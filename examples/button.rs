#![no_std]
#![no_main]

extern crate panic_itm;
use cortex_m_rt::entry;

use stm32f3_discovery::stm32f3xx_hal::delay::Delay;
use stm32f3_discovery::stm32f3xx_hal::pac;
use stm32f3_discovery::stm32f3xx_hal::prelude::*;

use stm32f3_discovery::button::UserButton;
use stm32f3_discovery::leds::Leds;
use stm32f3_discovery::switch_hal::{InputSwitch, OutputSwitch};

#[entry]
fn main() -> ! {
    let device_periphs = pac::Peripherals::take().unwrap();
    let mut reset_and_clock_control = device_periphs.RCC.constrain();

    let core_periphs = cortex_m::Peripherals::take().unwrap();
    let mut flash = device_periphs.FLASH.constrain();
    let clocks = reset_and_clock_control.cfgr.freeze(&mut flash.acr);
    let mut delay = Delay::new(core_periphs.SYST, clocks);

    // initialize user leds
    let mut gpioe = device_periphs.GPIOE.split(&mut reset_and_clock_control.ahb);
    let leds = Leds::new(
        gpioe.pe8,
        gpioe.pe9,
        gpioe.pe10,
        gpioe.pe11,
        gpioe.pe12,
        gpioe.pe13,
        gpioe.pe14,
        gpioe.pe15,
        &mut gpioe.moder,
        &mut gpioe.otyper,
    );
    let mut status_led = leds.ld3;

    // initialize user button
    let mut gpioa = device_periphs.GPIOA.split(&mut reset_and_clock_control.ahb);
    let button = UserButton::new(gpioa.pa0, &mut gpioa.moder, &mut gpioa.pupdr);

    loop {
        delay.delay_ms(50u16);

        match button.is_active() {
            Ok(true) => {
                status_led.on().ok();
            }
            Ok(false) => {
                status_led.off().ok();
            }
            Err(_) => {
                panic!("Failed to read button state");
            }
        }
    }
}
