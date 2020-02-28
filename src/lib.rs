#![no_std]

pub use accelerometer;
pub use lsm303dlhc;
pub use stm32f3xx_hal;
pub use switch_hal;

pub mod button;
pub mod compass;
pub mod leds;

/// Signals the process to go into low power mode until an interrupt occurs
pub fn wait_for_interrupt() {
    cortex_m::asm::wfi()
}
