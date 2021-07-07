#![no_std]

pub use accelerometer;

#[cfg(feature = "revD")]
pub use lsm303dlhc;

#[cfg(feature = "revE")]
pub use lsm303agr;


pub mod compass;


pub use stm32f3xx_hal;
pub use switch_hal;

pub mod button;

pub mod leds;

/// Signals the process to go into low power mode until an interrupt occurs
pub fn wait_for_interrupt() {
    cortex_m::asm::wfi()
}
