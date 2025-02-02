# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] - ReleaseDate

## [2.1.0] - 2024-03-27

### Added

- CLI argument for format `dat`

### Changed

- Updated library to version 2.1.0, see the changelog [here](https://git.mafiasi.de/21sbecker/assembler_lib/src/branch/main/CHANGELOG.md)
- The path to the file is now shown when there is a parser error in that file

## [2.0.0] - 2024-03-25

### Added

- Flag for writing output to `stdout`
  - This option cannot be used together with output path options
- Detect redirection and pipes for `stdout` and automatically set `stdout` flag for these situations
- Option for data output path

### Changed

- Documentation & Examples
  - Updated to reflect latest changes in version 2.0.0
- [BREAKING] Output option no longer dictates output path of data
  - `-t` or `--text-output` decides the path of the text output
  - `-d` or `--data-output` decides the path of the data output
  - For compatibility, `-o` and `--output` also decides the path of the text output
- [BREAKING] Updated library to version 2.0.0, see the changelog [here](https://git.mafiasi.de/21sbecker/assembler_lib/src/branch/main/CHANGELOG.md)
  - This update has breaking changes to the assembler syntax!
- [BREAKING] Moved library out of this crate into assembler_lib and use workspace for management
  - Does not impact program behavior but does impact development
- Use po4a as translation framework for documentation

### Removed

- [BREAKING] Debug format
  - Either use `RUST_LOG=debug` for more presentable information about instructions; or
  - Use the `--stdout` flag, which will write instructions and data to the terminal (if there is no redirection or piping)
- Integration tests that tested the library (which is done in the library already)

## [1.3.0] - 2024-03-03

### Added

- Python library
  - You can now use this library in python to work with assembly files!
  - There are two functions at your disposal:
    - `assemble(assembly_code, sp_init=true, no_nop_insert=false, format="mif", outpath=Path("a.mif"), comment=false, depth=1024, width=8)` which takes assembly code as a list of strings.
    - `assemble_paths(assembly_paths, sp_init=true, no_nop_insert=false, format="mif", outpath=Path("a.mif"), comment=false, depth=1024, width=8)` which first reads in assembly code from the `assembly_paths` list of paths and then calls `assemble` on it.
- Rust library
  - You can now use this crate in rust to work with assembly files!
- New documentation
  - This can be found in the directory "man" of the git repo
- Last instructions of RV32I for debugging & RARS compatibility
  - `ecall` - System call (should not be used, we do not have an OS)
  - `ebreak` - Breaking point for debugging purposes (should be used in debugger only)
- Automatic stack pointer initialization!
  - Stack pointer is automatically initialized to 4096 bytes
  - Use `--no-sp-init` to avoid initializing stack pointer
  - Note: When using `push` and `pop` the very bottom byte of the stack is not used. This may be subject to change!
- Initial support for label constants!
  - `.eqv <LABEL>, <IMMEDIATE>` can be used to set the label to that immediate
  - `li <REG>, <LABEL>` can be used to load that label into a register
- More tests
- Debug information for data parsing sub step
- Added more examples
  - `gcd.asm` which calculates the greatest common divisor of two signed numbers
  - `rng.asm` which generates random numbers with the xoroshiro128++ algorithm and a 128 bit seed
  - `fakultaet_128bit.asm` which uses 128 bits to store the solution of the faculty

### Changed

- Internal representation of some data structures
  - No changes in behavior
- 8 bit word width for MIF format being deprecated
  - It will not be deprecated!
- Information presentation in debugging log level
  - the information should be more concise
- panic behavior
  - A proper error is now thrown on edge cases, where panics or `std::process::exit` has been used before

### Deprecated

- Documentation in the gitea wiki
  - New documentation is in the "man" directory and readme of the git repo

### Fixed

- Data label after aligning halfs before words and dwords
  - This resulted in wrong labels afterwards. 

## [1.2.0] - 2024-02-10

### Added

- Initial & experimental support for constants and assembler directives!
  - Currently only initializing constants are supported with the `.byte`, `.half`, `.word`, `.dword`, `.ascii`, `.asciz` and `.string` identifier
  - To use constants you have to use sections, namely `.data` for constant declarations and `.text` for instructions
  - It is compatible to RARS, if you want to look there first!
  - If you specify `.data`, that data will be stored in a separate file with the same name but `<name>.mem.<ext>` 
  - Note: Comment has no effect on data mifs
- Simple progress bar for tracking... well, progress!
  - Progress bars are always written to stderr
- Simple Logging
  - There are multiple log levels: [error, warn, info, debug, trace, off] with error being the highest priority (always on unless log_level == off) and trace being the lowest priority (always off unless log_level == trace)
  - You can choose the log level by using the env "RUST_LOG", ex. `RUST_LOG=warn assembler ...`
  - Progress information and status messages are available via the `info` log level (default)
  - Logs are always written to stderr
  - Log level "debug" gives you more insight in the proceedings of the assembler, especially what instruction generates which machine code
- More test coverage
- New hardware supported instructions
  - `mulhu R1, R2, R3` - Unsigned high multiplication
  - `mulhsu R1, R2, R3` - Unsigned and signed high multiplication
  - `div R1, R2, R3` - Signed division
  - `divu R1, R2, R3` - Unsigned division (previously subroutine only)
  - `rem R1, R2, R3` - Signed remainder
  - `remu R1, R2, R3` - Unsigned remainder (previously subroutine only)
- More examples
  - `fakultaet_64bit.asm` which uses 64 bits to store the solution
  - `assembler_crasher_2000.asm` which is a very long assembly file **(Do NOT use debug log level)**
  - `hardware_test.asm` which is a program to test the hardware (end of execution, register x6 == 0xDEADBEEF)
- Better error reporting, altough still basic
- Taskfile.yaml for release automation (dev-ops)
- Debug output format
  - That format only prints to the console and will not write to any file
- Comment flag for mif format
  - Causes instructions to be included in human readable form as comments
  - Only works for mif format

### Changed

- Internal representation of some instructions (renamed)
  - No changes in behavior
- Internal structuring of modules
  - Also refactored some modules
  - No changes in behavior
- Panic behavior of functions
  - They now return errors and present them better to users!
- Mif format with 8 bit word width is now more compact

### Deprecated

- 8 bit word width for MIF format
  - We don't have a use for that and removing this feature could reduce the complexity of some functions

### Removed

- Multiplication subroutine and handling
  - Multiplication was hardware supported before and code has been commented out
  - No changes in behavior
- Division and Remainder subroutine and handling
  - These are now handled in hardware
  - No changes in behavior

### Fixed

- Running tests with all-features flag
- Bug that resulted in wrong jump and load addresses when using read after write nop insertion compilation flag
- Parsing bug that disallowed certain longer instructions because they were overshadowed (ex. lh before lhu)

## [1.1.2] - 2024-02-03

### Fixed

- Macro instructions with immediates that get expanded to multiple instructions
  - In certain situations these got optimized to one instruction instead of two, which resulted in wrong immediates
- Parsing instructions that also have shorter instructions as substring
  - There were instructions that could not get parsed because the shorter instructions have been matched incorrectly

## [1.1.1] - 2024-02-02

### Added

- Aarch64 (arm64) MacOS target for binaries

### Fixed

- Local labels not being found in nop insertion optimization step
- Func3 of Store instructions

## [1.1.0] - 2024-02-01

### Added

- Simple readme.md
- Cargo binstall metadata
  - binstall command is now shorter!
- Examples for assembly code under the "examples" directory
- New CLI arguments
  - `--format` or `-f` that specifies in which format the output should appear
    - Possible values are "raw" and "mif"
  - `--depth` that specifies the address count of the "mif" format file
    - Possible values are 1 to 65535
  - `--width` that specifies the width of a word
    - Possible values are 8 and 32
  - The default format is now "mif"
- Better error handling for output file writes
- Ability to write single line comments in the assembly code
  - Use '; < COMMENT >' to comment something in the code, works inline as well as before
  - Example:
    ```
    ; WORKS
        ; WORKS
    nop     ; WORKS
    ; WORKS LIKE A CHAMP ^^
    ```
- Repeat Macro, which can be used to.. well repeat instructions and macros!
  - Use 'rep < DEC >, [ MACRO | INSTRUCTION ]' to repeat the macro or instruction DEC amount of times
  - Nested repeat macros are not supported!
  - Example:
    ```
    ; WORKS
    rep 20, nop
    ; ERROR
    rep 20, rep 40, nop
    ```
- More tests

### Changed

- Default output path
  - Default output path of "mif" files is now "./a.mif"
  - Default output path of "raw" files is now "./a.bin"

### Fixed

- Writing output file if the file cannot be written to due to an error
  - File is now not written
- Bit masking for immediates in shift operations with immediates
  - This resulted in incorrect amount of shifts
- Endianness
  - Little endian is now used for binary output
- `call <LABL>` and `tail <LABL>`
  - Using these resulted in wrong jumps

## [1.0.1] - 2024-01-26

### Added

- Gitea Codeowners file for responsibility management
  - No changes in program behavior

### Changed

- Build option flags for the release build
  - Binaries are now significantly smaller & probably faster in execution as well
  - Release builds now take a little bit more time

### Removed

- A doc file from the repo, which contained outdated information
  - No changes in program behavior
- Part of help string for "--no-nop-insertion" flag, which stated that this option was not respected
  - The flag was respected before, so no changes in program behavior

### Fixed

- NOP insertion for load instructions
  - The wrong register was being used to look for hazards

## [1.0.0] - 2024-01-23

### Added

- Support for most of the RV32I instructions
  - No support for ecall and ebreak (yet)
- Subroutines for some instructions of the RV32M instruction set, that are not supported by hardware
  - remu
  - div (subject of name change in later releases)
- Some Macros for easier use, for example:
  - Stack operations (push, pop)
  - Load address (la), load immediate (li), call, tail, mv...
- Linker for linking multiple assembly files to one binary
  - Global and local (file only) labels
- Simple CLI
  - Currently three arguments: input (multiple paths), output (optional, one path), no-nop-insertion (flag)
- Basic Optimizer for automatic insertion of nop instructions to circumvent data dependencies
- Use of all hardware features available
  - Forwarding unit, Multiplication hardware instruction

<!-- next-url -->
[Unreleased]: https://git.mafiasi.de/21sbecker/assembler/compare/2.1.0...HEAD
[2.1.0]: https://git.mafiasi.de/21sbecker/assembler/compare/2.0.0...2.1.0
[2.0.0]: https://git.mafiasi.de/21sbecker/assembler/compare/1.3.0...2.0.0
[1.3.0]: https://git.mafiasi.de/21sbecker/assembler/compare/1.2.0...1.3.0
[1.2.0]: https://git.mafiasi.de/21sbecker/assembler/compare/1.1.2...1.2.0
[1.1.2]: https://git.mafiasi.de/21sbecker/assembler/compare/1.1.1...1.1.2
[1.1.1]: https://git.mafiasi.de/21sbecker/assembler/compare/1.1.0...1.1.1
[1.1.0]: https://git.mafiasi.de/21sbecker/assembler/compare/1.0.1...1.1.0
[1.0.1]: https://git.mafiasi.de/21sbecker/assembler/compare/1.0.0...1.0.1
[1.0.0]: https://git.mafiasi.de/21sbecker/assembler/compare/05d33c7556d2d3d08d2bc21aa930810ab19428c1...1.0.0
