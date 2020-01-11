# `stm32f3-discovery`

Board support package for the [STM32F3DISCOVERY][stm32f3discovery] board.

## Dependencies

To build embedded programs using this you'll need:

- Rust 1.31, 1.30-beta, nightly-2018-09-13 or a newer toolchain. e.g. `rustup
  default beta`

- `rust-std` components (pre-compiled `core` crate) for the ARM Cortex-M
  target.

``` console
$ cargo install cargo-generate
$ rustup target add thumbv7em-none-eabihf
```

For more info on working with embedded Rust, see the [Embedded Rust Book][book] and the [Discovery Book][discovery-book].

## VS Code

This repository includes launch configurations for debugging CortexM programs with Visual Studio Code in the `.vscode/` directory.  
See [.vscode/README.md](./.vscode/README.md) for more information.  

# License

This template is licensed under either of

- Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

<!-- references -->
[stm32f3discovery]: https://www.st.com/en/evaluation-tools/stm32f3discovery.html#
[book]: https://rust-embedded.github.io/book
[discovery-book]: https://rust-embedded.github.io/discovery/