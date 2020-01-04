# `stm32f3-discovery-quickstart`

Template to develop bare metal Rust applications for the [STM32F3DISCOVERY][stm32f3discovery] board 

## Dependencies

To build embedded programs using this template you'll need:

- Rust 1.31, 1.30-beta, nightly-2018-09-13 or a newer toolchain. e.g. `rustup
  default beta`

- The `cargo generate` subcommand. [Installation
  instructions][cargo-generate-install].

- `rust-std` components (pre-compiled `core` crate) for the ARM Cortex-M
  target.

``` console
$ cargo install cargo-generate
$ rustup target add thumbv7em-none-eabihf
```

## Using this template

**NOTE**: This is the very short version that only covers building programs. For
the long version, which additionally covers flashing, running and debugging
programs, check [the embedded Rust book][book] and [the Discover book][discovery-book].

``` console
$ cargo generate --git https://github.com/rubberduck203/stm32f3-discovery-quickstart
 Project Name: app
 Creating project called `app`...
 Done! New project created /tmp/app

$ cd app
$ cargo build
```

## VS Code

This template includes launch configurations for debugging CortexM programs with Visual Studio Code located in the `.vscode/` directory.  
See [.vscode/README.md](./.vscode/README.md) for more information.  
If you're not using VS Code, you can safely delete the directory from the generated project.

# License

This template is licensed under either of

- Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Attributions

This project is based on and forked from the [cortex-m-quickstart][cortex-m-quickstart] project.


<!-- references -->
[stm32f3discovery]: https://www.st.com/en/evaluation-tools/stm32f3discovery.html#
[cortex-m-quickstart]: https://github.com/rust-embedded/cortex-m-quickstart
[book]: https://rust-embedded.github.io/book
[discovery-book]: https://rust-embedded.github.io/discovery/
[cargo-generate-install]: https://github.com/ashleygwilliams/cargo-generate#installation