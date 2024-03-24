/*
 * Copyright (c) 2023 Steven Becker
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use assembler_lib::{
    asm::{translator::{CodeWriter, Format, MifFormat, RawFormat, WordWidth}, ParseLinkBuilder}, common::{errors::{ExitErrorCode, TranslatorError}, TranslatableCode}
};

use clap::{
    builder::ArgPredicate, crate_authors, crate_description, crate_version, value_parser, Arg, ArgAction, ArgMatches, Command, ValueHint
};
use indicatif_log_bridge::LogWrapper;
use log::{log_enabled, error};
use std::{
    fs, io::IsTerminal, path::PathBuf
};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use console::Term;

fn cli_interface() -> ArgMatches {
    #[allow(non_upper_case_globals)]
    Command::new("Assembler")
    .author(crate_authors!(", "))
    .version(crate_version!())
    .about(crate_description!())
    .help_template("\
{before-help}{name} - {version}
by {author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}

Copyright: MPL-2.0 (https://mozilla.org/MPL/2.0/)
")
    .arg(Arg::new("format")
                .value_hint(ValueHint::Other)
                .value_parser(["mif", "raw"])
                .action(ArgAction::Set)
                .short('f')
                .num_args(1)
                .default_value("mif")
                .long("format")
                .help("The format in which the output should be written in")
    )
    .arg(Arg::new("input")
                .value_names(["main asm file", "another asm file"])
                .value_hint(ValueHint::FilePath)
                .value_parser(value_parser!(PathBuf))
                .action(ArgAction::Set)
                .short('i')
                .num_args(1..=10)
                .required(true)
                .long("input")
                .help("Input assembly files, use \"<PATH>\"")
    )
    .arg(Arg::new("text_output")
                .value_name("output text file")
                .value_hint(ValueHint::FilePath)
                .value_parser(value_parser!(PathBuf))
                .action(ArgAction::Set)
                .short('t')
                .short_alias('o')
                .num_args(1)
                .default_value("a.bin")
                .default_value_if("format", ArgPredicate::Equals("raw".into()), Some("a.bin"))
                .default_value_if("format", ArgPredicate::Equals("mif".into()), Some("a.mif"))
                .required(false)
                .long("text-output")
                .alias("output")
                .help("The destination for the text output file")
    )
    .arg(Arg::new("data_output")
                .value_name("output data file")
                .value_hint(ValueHint::FilePath)
                .value_parser(value_parser!(PathBuf))
                .action(ArgAction::Set)
                .short('d')
                .num_args(1)
                .default_value("a.mem.bin")
                .default_value_if("format", ArgPredicate::Equals("raw".into()), Some("a.mem.bin"))
                .default_value_if("format", ArgPredicate::Equals("mif".into()), Some("a.mem.mif"))
                .required(false)
                .long("data-output")
                .help("The destination for the data output file")
    )
    .arg(Arg::new("format_depth")
                .value_name("address count")
                .value_hint(ValueHint::Other)
                .value_parser(value_parser!(u16).range(1..65536))
                .long("depth")
                .action(ArgAction::Set)
                .num_args(1)
                .default_value("1024")
                .help("Depth for MIF format. Does not do anything, if format != mif.")
    )
    .arg(Arg::new("format_width")
                .value_name("word width in bits")
                .value_hint(ValueHint::Other)
                .value_parser(["8", "32"])
                .long("width")
                .action(ArgAction::Set)
                .num_args(1)
                .default_value("32")
                .help("Width for MIF format. Does not do anything, if format != mif.")
    )
    .arg(Arg::new("nop_insert")
                .long("no-nop-insertion")
                .action(ArgAction::SetTrue)
                .required(false)
                .help("Disallow nop insertion")
    )
    .arg(Arg::new("sp_init")
                .long("no-sp-init")
                .action(ArgAction::SetFalse)
                .required(false)
                .help("Disallow stack pointer initialization")
    )
    .arg(Arg::new("comment_mif")
                .long("comment")
                .short('c')
                .action(ArgAction::SetTrue)
                .required(false)
                .help("Comment mif with used instructions. Does not do anything, if format != mif.")
    )
    .arg(Arg::new("stdout")
                .long("stdout")
                .action(ArgAction::SetFalse)
                .conflicts_with_all(["text_output", "data_output"])
                .required(false)
                .help("Write output to stdout")
    )
    .get_matches()
}

#[inline]
fn write_code<T: Format>(code_writer: CodeWriter<T>, not_stdout_out: bool, data_empty: bool, text_output: &PathBuf, data_output: &PathBuf) -> Result<(), TranslatorError> {
    match (not_stdout_out, data_empty) {
        (true, true) => code_writer.write_text_file(text_output),
        (true, false) => code_writer.write_files(text_output, data_output),
        (false, true) => code_writer.write_text_stdout(),
        (false, false) => code_writer.write_to(std::io::stdout()),
    }
}

fn translate(translatable_code: TranslatableCode, matches: ArgMatches) -> Result<String, TranslatorError> {
    // always returns Some(_)
    let outfmt = matches.get_one::<String>("format").unwrap();
    let text_output = matches.get_one::<PathBuf>("output").unwrap();
    let data_output = matches.get_one::<PathBuf>("output").unwrap();
    let comment = matches.get_flag("comment_mif");
    let depth = matches.get_one("format_depth").unwrap();
    let width = str::parse::<u8>(matches.get_one::<String>("format_width").unwrap()).unwrap();
    let not_stdout_out = matches.get_flag("stdout") || std::io::stdout().is_terminal();

    let data_empty = {
        translatable_code.get_all_ref().1.is_empty()
    };

    match outfmt.as_str() {
        "mif" => {
            let width = if width == 32 { WordWidth::ThirtyTwoBit } else { WordWidth::EightBit };
            let mif_format = MifFormat::default().set_comment(comment).set_mem_len(*depth).set_word_len(width);
            let code_writer = CodeWriter::new(mif_format, translatable_code);

            write_code(code_writer, not_stdout_out, data_empty, text_output, data_output)?;
        },
        "raw" => {
            let code_writer = CodeWriter::new(RawFormat, translatable_code);

            write_code(code_writer, not_stdout_out, data_empty, text_output, data_output)?;
        },
        "dat" => {
            error!("Dat format is currently not available!");
            std::process::exit(1)
        },
        _ => unreachable!(),
    };

    Ok(if not_stdout_out && data_empty {
        format!(
            "{:>12} {} ({})",
            console::Style::new().bold().apply_to("Assembled"),
            text_output.as_path().file_name().unwrap().to_str().unwrap(),
            std::fs::canonicalize(text_output.as_path()).unwrap().parent().unwrap().to_str().unwrap()
        )
    } else if not_stdout_out && !data_empty {
        format!(
            "{:>12} {} ({}, {})",
            console::Style::new().bold().apply_to("Assembled"),
            text_output.as_path().file_name().unwrap().to_str().unwrap(),
            std::fs::canonicalize(text_output.as_path()).unwrap().parent().unwrap().to_str().unwrap(),
            std::fs::canonicalize(data_output.as_path()).unwrap().parent().unwrap().to_str().unwrap()
        )
    } else {
        format!(
            "{:>12} {}",
            console::Style::new().bold().apply_to("Assembled"),
            "Printed to console"
        )
    })
}

fn main() {
    let logger =
        env_logger::Builder::from_env(
            env_logger::Env::default().default_filter_or("info"))
            .format_timestamp(None)
            .build();

    let multi = MultiProgress::new();

    LogWrapper::new(multi.clone(), logger)
        .try_init()
        .unwrap();

    let matches = cli_interface();

    let vals: Vec<&PathBuf> = matches.get_many::<PathBuf>("input")
    .unwrap()
    .collect();

    let progbar;

    if !log_enabled!(log::Level::Info) {
        progbar = ProgressBar::hidden();
    } else {
        progbar = multi.add(ProgressBar::new(5));
        progbar.set_style(
            ProgressStyle::with_template(
                if Term::stdout().size().1 > 80 {
                    "{prefix:>12.cyan.bold} [{bar:57}] {pos}/{len} {wide_msg}"
                } else {
                    "{prefix:>12.cyan.bold} [{bar:57}] {pos}/{len}"
                },
            )
            .unwrap()
            .progress_chars("=> "),
        );
    }

    progbar.set_prefix("Assembling");
    progbar.set_message("Reading assembly files...");

    let mut builder = ParseLinkBuilder::new()
                                                .progbar(&progbar)
                                                .sp_init(matches.get_flag("sp_init"))
                                                .no_nop_insert(matches.get_flag("nop_insert"));

    for file in vals {
        match fs::read_to_string(file.as_path()) {
            Ok(val) => builder.add_code(val),
            Err(msg) => {
                error!("Could not read '{}': {}", file.as_path().to_string_lossy(), msg);
                std::process::exit(66)
            },
        };
    }

    progbar.inc(1);
    progbar.set_message("Parsing...");

    let translatable_code = match builder.parse_link_optimize() {
        Ok(data) => data,
        Err(msg) => {
            error!("{msg}");
            std::process::exit(msg.get_err_code())
        }
    };

    progbar.inc(1);
    progbar.set_message("Translating & Writing...");

    let line = match translate(translatable_code, matches) {
        Ok(finish_line) => finish_line,
        Err(e) => {
            error!("{e}");
            std::process::exit(e.get_err_code())
        }
    };

    progbar.println(line);
    progbar.set_prefix("Finished");
    progbar.finish_with_message("Success");
}
 
