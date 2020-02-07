#![no_std]
#![no_main]

extern crate panic_itm;

use cortex_m_rt::entry;

use stm32f3_discovery::stm32f3xx_hal::delay::Delay;
use stm32f3_discovery::stm32f3xx_hal::prelude::*;
use stm32f3_discovery::stm32f3xx_hal::stm32;

use stm32f3_discovery::switch_hal::{OutputSwitch, ToggleableOutputSwitch};
use stm32f3_discovery::leds::Leds;
use stm32f3_discovery::lsm303dlhc::Lsm303dlhc;

#[entry]
fn main() -> ! {
    let device_periphs = stm32::Peripherals::take().unwrap();
    let mut reset_control_clock = device_periphs.RCC.constrain();

    let core_periphs = cortex_m::Peripherals::take().unwrap();
    let mut flash = device_periphs.FLASH.constrain();
    let clocks = reset_control_clock.cfgr.freeze(&mut flash.acr);
    let mut delay = Delay::new(core_periphs.SYST, clocks);

    // initialize user leds
    let gpioe = device_periphs.GPIOE.split(&mut reset_control_clock.ahb);
    let (_leds, _gpioe) = Leds::init(gpioe);

    /*
     * PB6 -> SCL (select clock)
     * PB7 -> SDA (?)
     * PE2 -> DRDY (data ready)
     * PE4 -> INT1
     * PE5 -> INT2
     */
    //let lsm303 = Lsm303dlhc::new(i2c: I2C);

    loop {

    }
}