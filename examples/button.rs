#![no_std]
#![no_main]

extern crate panic_itm;
use cortex_m_rt::entry;

use stm32f3_discovery::delay::Delay;
use stm32f3_discovery::prelude::*;
use stm32f3_discovery::stm32;

use stm32f3_discovery::leds::Led;
use stm32f3_discovery::button::{Button, UserButton};

#[entry]
fn main() -> ! {
    let device_periphs = stm32::Peripherals::take().unwrap();
    let mut reset_and_clock_control = device_periphs.RCC.constrain();

    let core_periphs = cortex_m::Peripherals::take().unwrap();
    let mut flash = device_periphs.FLASH.constrain();
    let clocks = reset_and_clock_control.cfgr.freeze(&mut flash.acr);
    let mut delay = Delay::new(core_periphs.SYST, clocks);

    // initialize user leds
    let gpioe = device_periphs.GPIOE.split(&mut reset_and_clock_control.ahb);
    let (leds, _gpioe) = stm32f3_discovery::leds::Leds::init(gpioe);
    let mut status_led = leds.ld3;

    // initialize user button
    let gpioa = device_periphs.GPIOA.split(&mut reset_and_clock_control.ahb);
    let button = UserButton::new(gpioa.pa0);

    loop {
        delay.delay_ms(50u16);

        match button.is_pressed() {
            Ok(true) => status_led.on(),
            Ok(false) => status_led.off(),
            Err(_e) => panic!("Failed to read button state")
        }
    }
}