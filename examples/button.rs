#![no_std]
#![no_main]

extern crate panic_itm;
use cortex_m_rt::entry;

use stm32f3_discovery::delay::Delay;
use stm32f3_discovery::prelude::*;
use stm32f3_discovery::stm32;

use stm32f3_discovery::leds::Led;

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
    let (leds, _gpioe) = stm32f3_discovery::leds::Leds::init(gpioe);
    let mut status_led = leds.ld3;

    // initialize user button
    let gpioa = device_periphs.GPIOA.split(&mut reset_control_clock.ahb);
    let button = gpioa.pa0; //defaults to Input<Floating>; Has an external pulldown & low pass filter.

    loop {
        delay.delay_ms(50u16);

        match button.is_high() {
            Ok(is_high) => {
                match is_high {
                    true => status_led.on(),
                    false => status_led.off()
                }
            },
            Err(_e) => panic!("Failed to read button state")
        }
    }
}
