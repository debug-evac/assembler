/*
 * Copyright (c) 2023 Steven Becker
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use assert_cmd::prelude::*;
use predicates::prelude::*;
use assert_fs::prelude::*;
use std::process::Command;
#[cfg(feature = "infer_stdout")]
use std::process::Stdio;

#[test]
fn test_input_not_existing() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("assembler")?;

    cmd.arg("-i").arg("non_existent_random_file_that_does_not_exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("[ERROR assembler] Could not read \'non_existent_random_file_that_does_not_exist\':"));

    Ok(())
}

#[test]
fn test_translate_test_file() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new()?;
    let input = temp.child("test_assembly_file.asm");
    input.write_str(r#"SmallExample: addi sp, zero, 0x300
li x25, 150006
push x25
pop x25
ret
"#)?;

    let output_path = temp.child("a.bin");

    let cor_output = temp.child("correct_output.bin");

    let mut cor_instr_vec: Vec<u8> = vec![];

    #[cfg(feature = "raw_nop")]
    let instr_vec: Vec<u32> = Vec::from([
        0b00_00000_00000_00000_00100_01001_10111, // Lui - sp init
        0b00_11000_00000_00000_00000_01000_10011,
        0b00_00000_00000_00100_10111_00101_10111, // Li - lui
        0b00_00000_00000_00000_00000_00000_10011, // 3x NOP
        0b00_00000_00000_00000_00000_00000_10011,
        0b00_00000_00000_00000_00000_00000_10011,
        0b10_01111_10110_11001_00011_00100_10011, // Li - addi
        0b11_11111_11100_00010_00000_01000_10011, // Push - addi -4
        0b00_00000_00000_00000_00000_00000_10011, // 3x NOP
        0b00_00000_00000_00000_00000_00000_10011,
        0b00_00000_00000_00000_00000_00000_10011,
        0b00_00000_11001_00010_11000_00001_00011,
        0b00_00000_00000_00010_01011_00100_00011, // Pop - Lw
        0b00_00000_00100_00010_00000_01000_10011, // Pop - addi +4
        0b00_00000_00000_00001_00000_00011_00111
    ]);

    #[cfg(not(feature = "raw_nop"))]
    let instr_vec: Vec<u32> = Vec::from([
        0b00_00000_00000_00000_00100_01001_10111, // Lui - sp init
        0b00_11000_00000_00000_00000_01000_10011,
        0b00_00000_00000_00100_10111_00101_10111, // Li - lui
        0b10_01111_10110_11001_00011_00100_10011, // Li - addi
        0b11_11111_11100_00010_00000_01000_10011, // Push - addi -4
        0b00_00000_11001_00010_11000_00001_00011,
        0b00_00000_00000_00010_01011_00100_00011, // Pop - Lw
        0b00_00000_00100_00010_00000_01000_10011,
        0b00_00000_00000_00001_00000_00011_00111
    ]);

    for instr in instr_vec.iter() {
        cor_instr_vec.extend(instr.to_le_bytes());
    }

    cor_output.write_binary(&cor_instr_vec)?;

    let mut cmd = Command::cargo_bin("assembler")?;
    cmd.arg("-i").arg(input.path()).arg("-o").arg(output_path.path()).arg("-f").arg("raw");

    #[cfg(feature = "infer_stdout")]
    cmd.stdout(Stdio::inherit())
        .assert()
        .success();

    #[cfg(not(feature = "infer_stdout"))]
    cmd.assert()
        .success();

    output_path
        .assert(predicate::path::is_file())
        .assert(predicate::path::eq_file(cor_output.path()));

    temp.close()?;

    Ok(())
}

#[test]
fn test_translate_multiple_files() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new()?;
    let main_in = temp.child("test_main.asm");
    main_in.write_str(r#"SmallExample: addi sp, zero, 0x300
li x25, 150006
push x25
call biggun
pop x25
ret
"#)?;

    let output_path = temp.child("a.bin");

    let sec_in = temp.child("test_sec.asm");
    sec_in.write_str(r#"biggun: lb x26, 0x500
ret
"#)?;

    let cor_output = temp.child("correct_output.bin");

    let mut cor_instr_vec: Vec<u8> = vec![];

    #[cfg(feature = "raw_nop")]
    let instr_vec: Vec<u32> = Vec::from([
        0b00_11000_00000_00000_00000_01000_10011,
        0b00_00000_00000_00100_10111_00101_10111, // Li - lui
        0b00_00000_00000_00000_00000_00000_10011, // 3x NOP
        0b00_00000_00000_00000_00000_00000_10011,
        0b00_00000_00000_00000_00000_00000_10011,
        0b10_01111_10110_11001_00011_00100_10011, // Li - addi
        0b11_11111_11100_00010_00000_01000_10011, // Push - addi -4
        0b00_00000_00000_00000_00000_00000_10011, // 3x NOP
        0b00_00000_00000_00000_00000_00000_10011,
        0b00_00000_00000_00000_00000_00000_10011,
        0b00_00000_11001_00010_11000_00001_00011, // Push - sw
        0b00_00000_00000_00000_00000_00100_10111, // Call - auipc - 11
        0b00_00000_00000_00000_00000_00000_10011, // 3x NOP
        0b00_00000_00000_00000_00000_00000_10011,
        0b00_00000_00000_00000_00000_00000_10011,
        0b00_00001_00000_00001_00000_00111_00111, // Call - jalr - 15
        0b00_00000_00000_00010_01011_00100_00011, // Pop - Lw
        0b00_00000_00100_00010_00000_01000_10011, // Pop - addi +4
        0b00_00000_00000_00001_00000_00011_00111, // ret
        0b01_01000_00000_00000_00011_01000_00011, // Lw - 19
        0b00_00000_00000_00001_00000_00011_00111, // ret
    ]);

    #[cfg(not(feature = "raw_nop"))]
    let instr_vec: Vec<u32> = Vec::from([
        0b00_00000_00000_00000_00100_01001_10111, // Lui - sp init
        0b00_11000_00000_00000_00000_01000_10011,
        0b00_00000_00000_00100_10111_00101_10111, // Li - lui
        0b10_01111_10110_11001_00011_00100_10011, // Li - addi
        0b11_11111_11100_00010_00000_01000_10011, // Push - addi -4
        0b00_00000_11001_00010_11000_00001_00011, // Push - sw
        0b00_00000_00000_00000_00000_00100_10111, // Call - auipc - 5
        0b00_00000_10100_00001_00000_00111_00111, // Call - jalr - 6
        0b00_00000_00000_00010_01011_00100_00011, // Pop - Lw
        0b00_00000_00100_00010_00000_01000_10011, // Pop - addi +4
        0b00_00000_00000_00001_00000_00011_00111, // ret
        0b01_01000_00000_00000_00011_01000_00011, // Lw - 10
        0b00_00000_00000_00001_00000_00011_00111, // ret
    ]);

    for instr in instr_vec.iter() {
        cor_instr_vec.extend(instr.to_le_bytes());
    }

    cor_output.write_binary(&cor_instr_vec)?;

    let mut cmd = Command::cargo_bin("assembler")?;
    #[cfg(feature = "raw_nop")]
    cmd.arg("-i").arg(main_in.path()).arg(sec_in.path())
                 .arg("-o").arg(output_path.path())
                 .arg("-f").arg("raw").arg("--no-sp-init");
    #[cfg(not(feature = "raw_nop"))]
    cmd.arg("-i").arg(main_in.path()).arg(sec_in.path())
                 .arg("-o").arg(output_path.path())
                 .arg("-f").arg("raw");

    #[cfg(feature = "infer_stdout")]
    cmd.stdout(Stdio::inherit())
        .assert()
        .success();

    #[cfg(not(feature = "infer_stdout"))]
    cmd.assert()
        .success();

    let get_vec = std::fs::read(output_path.path())?;

    for (counter, var) in get_vec.chunks(4).enumerate() {
        let read_val = u32::from_le_bytes(var.try_into().unwrap());
        let cor_val = *instr_vec.get(counter).unwrap();
        if cor_val != read_val {
            println!("Line: {}\nExpected: {:#034b}\nGot: {:#034b}", counter, cor_val, read_val);
        }
    }

    output_path
        .assert(predicate::path::is_file())
        .assert(predicate::path::eq_file(cor_output.path()));

    temp.close()?;

    Ok(())
}

#[test]
fn test_translate_no_nop() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new()?;
    let input = temp.child("test_assembly_file.asm");
    input.write_str(r#"SmallExample: addi sp, zero, 0x300
li x25, 150006
push x25
pop x25
ret
"#)?;

    let output_path = temp.child("a.bin");

    let cor_output = temp.child("correct_output.bin");

    let mut cor_instr_vec: Vec<u8> = vec![];

    let instr_vec: Vec<u32> = Vec::from([
        0b00_00000_00000_00000_00100_01001_10111, // Lui - sp init
        0b00_11000_00000_00000_00000_01000_10011,
        0b00_00000_00000_00100_10111_00101_10111, // Li - lui
        0b10_01111_10110_11001_00011_00100_10011, // Li - addi
        0b11_11111_11100_00010_00000_01000_10011, // Push - addi -4
        0b00_00000_11001_00010_11000_00001_00011,
        0b00_00000_00000_00010_01011_00100_00011, // Pop - Lw
        0b00_00000_00100_00010_00000_01000_10011,
        0b00_00000_00000_00001_00000_00011_00111
    ]);

    for instr in instr_vec.iter() {
        cor_instr_vec.extend(instr.to_le_bytes());
    }

    cor_output.write_binary(&cor_instr_vec)?;

    let mut cmd = Command::cargo_bin("assembler")?;
    cmd.arg("-i").arg(input.path()).arg("-o").arg(output_path.path())
                                   .arg("--no-nop-insertion")
                                   .arg("-f").arg("raw");

    #[cfg(feature = "infer_stdout")]
    cmd.stdout(Stdio::inherit())
        .assert()
        .success();

    #[cfg(not(feature = "infer_stdout"))]
    cmd.assert()
        .success();

    output_path
        .assert(predicate::path::is_file())
        .assert(predicate::path::eq_file(cor_output.path()));

    temp.close()?;

    Ok(())
}

#[test]
fn test_translate_fail_undef_label() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new()?;

    let input = temp.child("test_label_undefined.asm");
    input.write_str(r#"call nonExistentLabel
ret
"#)?;

    let mut cmd = Command::cargo_bin("assembler")?;

    cmd.arg("-i").arg(input.path()).arg("-o").arg(temp.path().join("a.bin"));
    cmd.assert()
        .failure();

    Ok(())
}

#[test]
fn test_translate_fail_unknown_instr() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new()?;

    let input = temp.child("test_wrong_instr.asm");
    input.write_str(r#"aylmao
ret
"#)?;

    let mut cmd = Command::cargo_bin("assembler")?;

    cmd.arg("-i").arg(input.path()).arg("-o").arg(temp.path().join("a2.bin"));
    cmd.assert()
        .failure();

    Ok(())
}

#[test]
fn test_faraway_calls() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new()?;

    let output_path = temp.child("a.bin");

    let input = temp.child("test_far_call.asm");
    input.write_str(r#"
call farAway
rep 1024, nop
farAway: j farAway
"#)?;

    let cor_output = temp.child("correct_output.bin");

    let mut cor_instr_vec: Vec<u8> = vec![];

    #[cfg(feature = "raw_nop")]
    let mut instr_vec: Vec<u32> = Vec::from([
        0b00_00000_00000_00000_00100_00100_10111, // auipc call farAway
        0b00_00000_00000_00000_00000_00000_10011,
        0b00_00000_00000_00000_00000_00000_10011,
        0b00_00000_00000_00000_00000_00000_10011,
        0b00_00000_11000_00001_00000_00111_00111, // jalr
    ]);

    #[cfg(not(feature = "raw_nop"))]
    let mut instr_vec: Vec<u32> = Vec::from([
        0b00_00000_00000_00000_00100_01001_10111, // Lui - sp init
        0b00_00000_00000_00000_00100_00100_10111, // auipc call farAway
        0b00_00000_01100_00001_00000_00111_00111, // jalr
    ]);

    for _ in 0..1024 {
        instr_vec.push(0b00_00000_00000_00000_00000_00000_10011); // nop
    }

    instr_vec.push(0b00_00000_00000_00000_00000_00011_01111); // jal

    for instr in instr_vec.iter() {
        cor_instr_vec.extend(instr.to_le_bytes());
    }

    cor_output.write_binary(&cor_instr_vec)?;

    let mut cmd = Command::cargo_bin("assembler")?;

    #[cfg(feature = "raw_nop")]
    cmd.arg("-i").arg(input.path()).arg("-o").arg(output_path.path())
                 .arg("-f").arg("raw").arg("--no-sp-init");
    #[cfg(not(feature = "raw_nop"))]
    cmd.arg("-i").arg(input.path()).arg("-o").arg(output_path.path())
                 .arg("-f").arg("raw");

    #[cfg(feature = "infer_stdout")]
    cmd.stdout(Stdio::inherit())
        .assert()
        .success();

    #[cfg(not(feature = "infer_stdout"))]
    cmd.assert()
        .success();

    output_path
        .assert(predicate::path::is_file())
        .assert(predicate::path::eq_file(cor_output.path()));

    temp.close()?;

    Ok(())
}