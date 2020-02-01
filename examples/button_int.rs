#![no_std]
#![no_main]

extern crate panic_itm;

use cortex_m_rt::entry;

use stm32f3_discovery::prelude::*;
use stm32f3_discovery::stm32;

use core::sync::atomic::{AtomicBool, Ordering};
use stm32f3_discovery::button;
use stm32f3_discovery::interrupt;
use stm32f3_discovery::leds::Led;

static USER_BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);

#[interrupt]
fn EXTI0() {
    // pa0 has a low pass filter on it, so no need to debounce in software
    USER_BUTTON_PRESSED.store(true, Ordering::Relaxed);
    //If we don't clear the interrupt to signal it's been serviced, it will continue to fire.
    button::interrupt::clear();
}

#[entry]
fn main() -> ! {
    let device_periphs = stm32::Peripherals::take().unwrap();
    let mut reset_and_clock_control = device_periphs.RCC.constrain();

    // initialize user leds
    let gpioe = device_periphs.GPIOE.split(&mut reset_and_clock_control.ahb);
    let (leds, _gpioe) = stm32f3_discovery::leds::Leds::init(gpioe);
    let mut status_led = leds.ld3;

    button::interrupt::enable(&device_periphs.EXTI, &device_periphs.SYSCFG);

    loop {
        // check to see if flag was active and clear it
        if USER_BUTTON_PRESSED.swap(false, Ordering::AcqRel) {
            Led::toggle(&mut status_led);
        }
    }
}