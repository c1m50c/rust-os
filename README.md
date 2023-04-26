# rust-os

[![Verify](https://github.com/c1m50c/rust-os/actions/workflows/verify.yaml/badge.svg?branch=main&event=push)](https://github.com/c1m50c/rust-os/actions/workflows/verify.yaml)

Repository following the ["Writing an Operating System in Rust"] blog by Philipp Oppermann. 

["Writing an Operating System in Rust"]: https://os.phil-opp.com/

## Installing Requirements

### Compilation Tools

This operating system is written in Rust so you'll need to install the language's compilation tools by running the following commmands:

#### Windows

```bash
$ choco install rust
```

#### MacOS

```bash
$ brew install rust
```

### Nightly Channel

Alongside the normal compilation toolkit you'll need to add the [`nightly`] channel to your [`rustup`] installation, you can do so by running the following commands:

```
$ rustup update
> ...

$ rustup toolchain install nightly
> ...

# Also add the `x86_64-unknown-none` build target for compiling our kernel.
$ rustup target add x86_64-unknown-none
```

[`rustup`]: https://rust-lang.github.io/rustup/index.html
[`nightly`]: https://rust-lang.github.io/rustup/concepts/channels.html#working-with-nightly-rust

### QEMU

Lastly, you'll need to install [QEMU] to emulate our operating system. To do so, follow the [installation instructions] for the platform you're currently using.

[QEMU]: https://www.qemu.org/
[installation instructions]: https://www.qemu.org/download/

## Running

After you've installed all of the requirements, running the operating system should be as simple as executing the following commands:

```bash
$ cargo run --release -- <uefi||bios>
> ... # QEMU Window should pop up
```