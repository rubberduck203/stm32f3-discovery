#![no_std]
#![no_main]

extern crate panic_itm;

use cortex_m_rt::entry;

use stm32f3_discovery::stm32f3xx_hal::interrupt;
use stm32f3_discovery::stm32f3xx_hal::pac;
use stm32f3_discovery::stm32f3xx_hal::prelude::*;
use stm32f3_discovery::wait_for_interrupt;

use core::sync::atomic::{AtomicBool, Ordering};
use stm32f3_discovery::button;
use stm32f3_discovery::button::interrupt::TriggerMode;

use stm32f3_discovery::leds::Leds;
use stm32f3_discovery::switch_hal::ToggleableOutputSwitch;

static USER_BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);

#[interrupt]
fn EXTI0() {
    //If we don't clear the interrupt to signal it's been serviced, it will continue to fire.
    button::interrupt::clear();
    // pa0 has a low pass filter on it, so no need to debounce in software
    USER_BUTTON_PRESSED.store(true, Ordering::Relaxed);
}

#[entry]
fn main() -> ! {
    let device_periphs = pac::Peripherals::take().unwrap();
    let mut reset_and_clock_control = device_periphs.RCC.constrain();

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

    button::interrupt::enable(
        &device_periphs.EXTI,
        &device_periphs.SYSCFG,
        TriggerMode::Rising,
    );

    loop {
        // check to see if flag was active and clear it
        if USER_BUTTON_PRESSED.swap(false, Ordering::AcqRel) {
            status_led.toggle().ok();
        }

        wait_for_interrupt();
    }
}
