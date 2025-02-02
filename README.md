# Assembler

This Assembler was programmed within the project "Mikrorechner" in order to run assembly code on a self-designed CPU.
It uses a modified MIPS assembly syntax and features labels, macros, automatic nop insertion, a simple linker, instruction minimization for certain macros and output formatting to raw, mif or dat.

## Features

- [x] Support for all instructions of the RV32I and RV32M extensions
- [x] Support for some macros for easier programming, especially:
  - [x] Stack operations (push, pop)
  - [x] Load address (la), load immediate (li), call, tail & mv
- [x] Simple linker to link multiple input assembly files
  - [x] Global and local (file scope) labels
- [x] Rudimentary optimizer for automatic insertions of nop instructions when data hazards are expected
- [x] Rudimentary user output (wip)
- [x] Support for assembler directives that are used to store constants in memory
- [x] Python module for easier integration into the compiler
- [x] Rudimentary error reporting
- [ ] Better internal code documentation
- [ ] Disassembler-Mode (planned for a minor release of 2.0.0)

## Out of scope

These features may or may not be implemented.

- Simple peephole optimization, that for example maps `add x7, x6, x6` to `slli x7, x6, 1` (since shifts are faster than additions)

## Installation

There are several ways to install the assembler. We recommend using the first method outlined as it is the fastest and safest method available.

### Binary

#### Using `cargo binstall`

[Cargo binstall](https://github.com/cargo-bins/cargo-binstall) is a program to install binaries from Github-like platforms without needing to compile them from source & without manual intervention. It is highly recommended to use this method.

Once installed, you need to run the following:

```
cargo binstall assembler --git https://git.mafiasi.de/21sbecker/assembler --only-signed
```

Then enter your username & password for Gitea. The installation is done.

Note: You cannot install a particular version with this method. If you need to use a particular version for whatever reason, we recommend compiling from source using the `cargo install` with Cargo registry method.

#### Using bare binaries

This method is not recommended as you have to manage updates and paths yourself. Head to the [releases](https://git.mafiasi.de/21sbecker/assembler/releases/latest) page and download the binary for your operating system. If there is no binary for your operating system, either open an issue [here](https://git.mafiasi.de/21sbecker/assembler/issues) or compile it yourself.

To use the binary, you either have it in the directory you are in (not ideal) or put it in the path that your system is using. You may need to look up how to put the binary in the path of your system.

### From Source 

If you have or want to compile the assembler from source, then you need to install cargo. See [this](https://www.rust-lang.org/tools/install) for instructions on how to install it. Once installed, you can use either of these methods to build the assembler from source. We recommend using the Cargo registry, which is a bit more involved, however ensures best compatibility and usability. When using `cargo install`, you may need to add the path of the bin directory to the system path variable. Please look up how to do that for your system.

#### Using `cargo install` with Cargo registry

To use the Cargo registry, you need to create two configuration files. The process is described for Linux. If you are using another operating system, you will need to modify some steps to make them fit.

##### Linux

Create a file at `~/.cargo/config.toml` with the following content:

```
[registry]
global-credential-providers = ["cargo:token", "cargo:libsecret"]

[registries.mafiasi-gitea] 
index = "sparse+https://git.mafiasi.de/api/packages/21sbecker/cargo/"
```

This defines a new registry to use for cargo commands and authentication methods for these registries. `Crate.io` is the default registry, which is **NOT USED** for this crate. If you do not have a system keyring installed, remove `"cargo:libsecret"` from the credential-providers.

Create another file at `~/.cargo/credentials.toml` with the following content:

```
[registries.mafiasi-gitea]
token = "Bearer {token}"
```

You need to create a token in Gitea and replace the placeholder with it. You can create a token for Mafiasi Gitea under this URL: [https://git.mafiasi.de/user/settings/applications](https://git.mafiasi.de/user/settings/applications). The token needs to have read/write permission for package. **DO NOT SHARE THE TOKEN WITH OTHERS!**

Once done, you can now use `cargo install assembler --registry mafiasi-gitea`. If you want to install a particular version, use `cargo install assembler@VERSION --registry mafiasi-gitea`, example `cargo install assembler@1.0.0 --registry mafiasi-gitea` for Version 1.0.0.

To uninstall the assembler, use `cargo uninstall assembler`.

#### Using `cargo install` with Git

This method is highly discouraged. You can clone this repo to your PC by using `git clone https://git.mafiasi.de/21sbecker/assembler -b stable`. We do not recommend using any other branch than stable.

You can then use `cargo install --path .` to compile the assembler from source and install it.

## Usage

```
$ assembler --help

Assembler - 2.1.0
by Steven Becker <steven.becker@studium.uni-hamburg.de>, Jan Julius <jan.julius@studium.uni-hamburg.de>
An assembler for a self-written RISC-V based CPU

Usage: assembler [OPTIONS] --input <main asm file> <another asm file>...

Options:
  -f, --format <format>
          The format in which the output should be written in [default: mif] [possible values: mif, raw, dat]
  -i, --input <main asm file> <another asm file>...
          Input assembly files, use "<PATH>"
  -t, --text-output <output text file>
          The destination for the text output file [default: a.bin]
  -d, --data-output <output data file>
          The destination for the data output file [default: a.mem.bin]
      --depth <address count>
          Depth for MIF format. Does not do anything, if format != mif. [default: 1024]
      --width <word width in bits>
          Width for MIF format. Does not do anything, if format != mif. [default: 32] [possible values: 8, 32]
      --no-nop-insertion
          Disallow nop insertion
      --no-sp-init
          Disallow stack pointer initialization
  -c, --comment
          Comment mif with used instructions. Does not do anything, if format != mif.
      --stdout
          Write output to stdout
  -h, --help
          Print help
  -V, --version
          Print version

Copyright: MPL-2.0 (https://mozilla.org/MPL/2.0/)
```

For more information, please head to the [man directory of this repo](https://git.mafiasi.de/21sbecker/assembler/src/branch/main/man).

## Support

Please open an issue if you encounter any bugs or irregularities and include information on how to reproduce that. We will promptly fix it then!

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

### Translations

We are using [po4a](https://po4a.org/) as translation framework for external documentation. If you want to provide translations in your language, simply create a .po file in your language or add to existing .po files.

## License

[MPL-2.0](https://www.mozilla.org/en-US/MPL/2.0/)
