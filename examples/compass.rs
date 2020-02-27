#![no_std]
#![no_main]

extern crate panic_itm;

use cortex_m::iprintln;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};

use stm32f3xx_hal::i2c::I2c;

use stm32f3_discovery::compass::Compass;
use stm32f3_discovery::lsm303dlhc::Lsm303dlhc;
use stm32f3_discovery::stm32f3xx_hal::prelude::*;
use stm32f3_discovery::stm32f3xx_hal::stm32;
use stm32f3_discovery::wait_for_interrupt;

#[entry]
fn main() -> ! {
    let device_periphs = stm32::Peripherals::take().unwrap();
    let mut reset_control_clock = device_periphs.RCC.constrain();

    let mut core_periphs = cortex_m::Peripherals::take().unwrap();
    let mut flash = device_periphs.FLASH.constrain();
    let clocks = reset_control_clock.cfgr.freeze(&mut flash.acr);

    // setup 1 second systick
    let mut syst = core_periphs.SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(8_000_000); // period = 1s
    syst.enable_counter();
    syst.enable_interrupt();

    // setup ITM output
    let stim = &mut core_periphs.ITM.stim[0];

    /*
     * PB6 -> SCL (clock)
     * PB7 -> SDA (data)
     * PE2 -> DRDY (magnometer data ready)
     * PE4 -> INT1 (configurable interrupt 1)
     * PE5 -> INT2 (configurable interrupt 2)
     */
    let mut gpiob = device_periphs.GPIOB.split(&mut reset_control_clock.ahb);

    // new lsm303 driver uses continuous mode, so no need wait for interrupts on DRDY
    let compass = Compass::new(
        gpiob.pb6,
        gpiob.pb7,
        &mut gpiob.moder,
        &mut gpiob.afrl,
        device_periphs.I2C1,
        clocks,
        &mut reset_control_clock.apb1
    ).unwrap();

    let mut lsm303dlhc = compass.lsm303dlhc;

    loop {
        let accel = lsm303dlhc.accel().unwrap();
        let mag = lsm303dlhc.mag().unwrap();

        iprintln!(stim, "Accel:{:?}; Mag:{:?}", accel, mag);

        wait_for_interrupt();
    }
}

#[exception]
fn SysTick() {
    // make sure we don't compile away
    cortex_m::asm::nop();
}
