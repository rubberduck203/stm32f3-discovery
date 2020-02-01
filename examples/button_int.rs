#![no_std]
#![no_main]

extern crate panic_itm;

use cortex_m::peripheral::NVIC;
use cortex_m_rt::entry;

use stm32f3_discovery::prelude::*;
use stm32f3_discovery::stm32;

use core::sync::atomic::{AtomicBool, Ordering};
use stm32::Interrupt;
use stm32f3_discovery::interrupt;

use stm32f3_discovery::leds::Led;

#[entry]
fn main() -> ! {
    let device_periphs = stm32::Peripherals::take().unwrap();
    let mut reset_and_clock_control = device_periphs.RCC.constrain();

    // initialize user leds
    let gpioe = device_periphs.GPIOE.split(&mut reset_and_clock_control.ahb);
    let (leds, _gpioe) = stm32f3_discovery::leds::Leds::init(gpioe);
    let mut status_led = leds.ld3;

    enable_button_interrupt(&device_periphs.EXTI, &device_periphs.SYSCFG);

    loop {
        // check to see if flag was active and clear it
        if USER_BUTTON_PRESSED.swap(false, Ordering::AcqRel) {
            Led::toggle(&mut status_led);
        }
    }
}

static USER_BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);

#[interrupt]
fn EXTI0() {
    // pa0 has a low pass filter on it, so no need to debounce in software
    USER_BUTTON_PRESSED.store(true, Ordering::Relaxed);
    clear_interrupt_pr0();
}

fn clear_interrupt_pr0() {
    const EXTI_PR1: usize = 0x40010414;
    const PR0: usize = (1 << 0);
    unsafe{
        core::ptr::write_volatile(EXTI_PR1 as *mut usize, PR0);
    }
}

fn enable_button_interrupt(external_interrupts: &stm32::EXTI, sysconfig: &stm32::SYSCFG) {
    // enable exti0
    let interrupt_mask_reg = &external_interrupts.imr1;
    interrupt_mask_reg.modify(|_, w| w.mr0().set_bit());

    //TODO: take enum to specify trigger mode {rising, falling, both}
    // trigger on rising edge
    let rising_trigger_select_reg = &external_interrupts.rtsr1;
    rising_trigger_select_reg.modify(|_, w| w.tr0().set_bit());

    // map line EXTI0 to PA0
    let external_interrupt_config = &sysconfig.exticr1;
    const PORT_A_CONFIG: u8 = 0x000;
    external_interrupt_config.modify(|_, w| unsafe { w.exti0().bits(PORT_A_CONFIG) });

    //enable interrupts on EXTI0
    unsafe {
        NVIC::unmask(Interrupt::EXTI0);
    }
}
