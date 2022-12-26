#![no_std]
#![no_main]

extern crate panic_itm;

use core::cell::RefCell;
use core::ops::DerefMut;

use cortex_m::interrupt::{free, Mutex};

use cortex_m_rt::entry;

use stm32f3xx_hal::pac;
use stm32f3xx_hal::prelude::*;
use stm32f3xx_hal::timer::{Event, Timer};

use pac::{interrupt, Interrupt};

use switch_hal::{IntoSwitch, ToggleableOutputSwitch};

use stm32f3_discovery::wait_for_interrupt;

static TIM: Mutex<RefCell<Option<Timer<pac::TIM7>>>> = Mutex::new(RefCell::new(None));

#[interrupt]
fn TIM7() {
    free(|cs| {
        if let Some(ref mut tim7) = TIM.borrow(cs).borrow_mut().deref_mut() {
            tim7.clear_event(Event::Update);
        }
    });
}

#[entry]
fn main() -> ! {
    let peripherals = stm32f3xx_hal::pac::Peripherals::take().unwrap();
    let mut flash = peripherals.FLASH.constrain();
    let mut rcc = peripherals.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut timer = Timer::new(peripherals.TIM7, clocks, &mut rcc.apb1);
    timer.start(500u32.milliseconds());
    timer.enable_interrupt(Event::Update);
    free(|cs| {
        TIM.borrow(cs).replace(Some(timer));
    });

    let mut gpio = peripherals.GPIOE.split(&mut rcc.ahb);
    let pin = gpio
        .pe9
        .into_push_pull_output(&mut gpio.moder, &mut gpio.otyper);
    let mut led = pin.into_active_high_switch();

    unsafe {
        pac::NVIC::unmask(Interrupt::TIM7);
    }

    loop {
        led.toggle().ok();
        wait_for_interrupt();
    }
}
