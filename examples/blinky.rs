#![no_std]
#![no_main]

extern crate panic_itm;

use cortex_m_rt::entry;

use stm32f3_discovery::stm32f3xx_hal::delay::Delay;
use stm32f3_discovery::stm32f3xx_hal::prelude::*;
use stm32f3_discovery::stm32f3xx_hal::stm32;

use stm32f3_discovery::leds::Leds;
use stm32f3_discovery::switch_hal::{OutputSwitch, ToggleableOutputSwitch};

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
    let (mut leds, _gpioe) = Leds::init(gpioe);

    loop {
        leds.ld3.toggle().ok();
        delay.delay_ms(1000u16);
        leds.ld3.toggle().ok();
        delay.delay_ms(1000u16);

        //explicit on/off
        leds.ld4.on().ok();
        delay.delay_ms(1000u16);
        leds.ld4.off().ok();
        delay.delay_ms(1000u16);
    }
}
