# `stm32f3-discovery`

Board support package for the [STM32F3DISCOVERY][stm32f3discovery] board.

![Rust](https://github.com/rubberduck203/stm32f3-discovery/workflows/Rust/badge.svg)
[![crates.io](https://img.shields.io/crates/d/stm32f3-discovery.svg)](https://crates.io/crates/stm32f3-discovery)
[![crates.io](https://img.shields.io/crates/v/stm32f3-discovery.svg)](https://crates.io/crates/stm32f3-discovery)
[![docs.rs](https://docs.rs/stm32f3-discovery/badge.svg)](https://docs.rs/stm32f3-discovery)

## Dependencies

To build embedded programs using this you'll need:

- Rust 1.51 or newer toolchain
- `rust-std` components (pre-compiled `core` crate) for the ARM Cortex-M
  target.

``` console
$ cargo install cargo-generate
$ rustup target add thumbv7em-none-eabihf
```

For more info on working with embedded Rust, see the [Embedded Rust Book][book] and the [Discovery Book][discovery-book].

## Documentation

https://docs.rs/stm32f3-discovery

For the board specific functionality this crate adds, see:
 - The [examples directory](./examples).
 - The [leds module documentation](https://docs.rs/stm32f3-discovery/0.3.4/stm32f3_discovery/leds/index.html)
 - The [button module documentation](https://docs.rs/stm32f3-discovery/0.3.4/stm32f3_discovery/button/index.html)
 - The [compass module](https://docs.rs/stm32f3-discovery/0.3.4/stm32f3_discovery/compass/index.html) and [lsm303dhlc documentation](https://docs.rs/lsm303dlhc/0.2.0/lsm303dlhc/)

## VS Code

This repository includes launch configurations for debugging CortexM programs with Visual Studio Code in the `.vscode/` directory.  
See [.vscode/README.md](./.vscode/README.md) for more information.  

To debug one of the examples, open the example source file in the editor and press F5.

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

## Changelog

### 0.7.2

Implements the [Accelerometer trait](https://docs.rs/accelerometer/latest/accelerometer/trait.Accelerometer.html) from the [Accelerometer crate](https://crates.io/crates/accelerometer).

### 0.7.1

- Implements an `Iterator` for the `Leds` struct and introduced the [Leds::iter_mut()](https://docs.rs/stm32f3-discovery/0.7.1/stm32f3_discovery/leds/struct.Leds.html#method.iter_mut) method.

    Testing shows that direct iteration over the leds using `Leds::iter_mut()` can save up to 800 bytes off the size of the final binary over the old `Leds::into_array()` method.

    See: https://github.com/rubberduck203/stm32f3-discovery/pull/41

- Adds the ability to obtain a mutable reference to a led based on it's compass direction on the board. 

    See: [Leds::for_direction()](https://docs.rs/stm32f3-discovery/0.7.1/stm32f3_discovery/leds/struct.Leds.html#method.for_direction)

    Contributed by [Christian Meusel](https://github.com/sirhcel)

### 0.7.0

Updates `stm32f3xx-hal` to 0.7.0.
Since we re-export the `stm32f3xx-hal`, any breaking changes in their API are also breaking changes in ours.
For details see the [stm32f3xx-hal changelog](https://github.com/stm32-rs/stm32f3xx-hal/blob/66c0d21ae19ae0bee09ec834a6c9c90b2191e17d/CHANGELOG.md#breaking-changes)


Although the minimum Rust version is technically still 1.49, because of changes to `embedded_time`, the minimum version of Cargo is now 1.51, so we're updating our MSRV to 1.51.

### 0.6.1

Update `cortex-m`, `cortex-m-rt`, and `switch-hal` dependencies.

### 0.6.0

Update `stm32f3xx-hal` version.  
`stm32f3xx-hal` had breaking changes.  
Since we re-export the HAL, that means we also had breaking changes.

For details, see the [stm32f3xx-hal changelog](https://github.com/stm32-rs/stm32f3xx-hal/blob/HEAD/CHANGELOG.md#breaking-changes).

### 0.5.0

- Updated dependencies
- `InputSwitch for UserButton` now has an `Error` type of `core::convert::Infallible` instead of `()`

### 0.4.0

- Updated `stm32f3xx-hal` from 0.4.0 to 0.4.1
- Allows setting `TriggerMode` on the user button (breaking change)
- Removes deprecated `GpioE` struct and `Leds::init` function

### 0.3.4

- Introduced `Compass` struct and implemented [Accelerometer trait](https://crates.io/crates/accelerometer).
- Add `Leds::new` function and deprecate `Leds::init`.

### 0.3.3

- Add `wait_for_interrupt` function
- Upgrade `switch-hal` version

### 0.3.2

- Re-export `lsm303dhlc` driver