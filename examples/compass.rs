#![no_std]
#![no_main]

extern crate panic_itm;

use cortex_m::iprintln;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};

use accelerometer::{Accelerometer, RawAccelerometer};
use stm32f3_discovery::compass::Compass;
use stm32f3_discovery::stm32f3xx_hal::pac;
use stm32f3_discovery::stm32f3xx_hal::prelude::*;
use stm32f3_discovery::wait_for_interrupt;

#[entry]
fn main() -> ! {
    let device_periphs = pac::Peripherals::take().unwrap();
    let mut reset_and_clock_control = device_periphs.RCC.constrain();

    let mut core_periphs = cortex_m::Peripherals::take().unwrap();
    let mut flash = device_periphs.FLASH.constrain();
    let clocks = reset_and_clock_control.cfgr.freeze(&mut flash.acr);

    // setup 1 second systick
    let mut syst = core_periphs.SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(8_000_000); // period = 1s
    syst.enable_counter();
    syst.enable_interrupt();

    // setup ITM output
    let stim = &mut core_periphs.ITM.stim[0];

    let mut gpiob = device_periphs.GPIOB.split(&mut reset_and_clock_control.ahb);

    // new lsm303 driver uses continuous mode, so no need wait for interrupts on DRDY
    let mut compass = Compass::new(
        gpiob.pb6,
        gpiob.pb7,
        &mut gpiob.moder,
        &mut gpiob.otyper,
        &mut gpiob.afrl,
        device_periphs.I2C1,
        clocks,
        &mut reset_and_clock_control.apb1,
    )
    .unwrap();

    loop {
        let accel = compass.accel_raw().unwrap();
        iprintln!(stim, "RawAccel:{:?}", accel);

        let normalized_accel = compass.accel_norm().unwrap();
        iprintln!(stim, "G-Force:{:?}", normalized_accel);

        let mag = compass.mag_raw().unwrap();
        iprintln!(stim, "RawMag:{:?}", mag);

        wait_for_interrupt();
    }
}

#[exception]
fn SysTick() {
    // make sure we don't compile away
    cortex_m::asm::nop();
}
