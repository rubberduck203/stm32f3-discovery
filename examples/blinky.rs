#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                         // extern crate panic_abort; // requires nightly
                         // extern crate panic_itm; // logs messages over ITM; requires ITM support
                         // extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

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
    let mut leds = stm32f3_discovery::leds::Leds::init(gpioe);

    loop {
        //use this syntax instead of ld3.toggle() to disambiuate from ToggleableOutputPin
        //TODO: import just what we need to expose?
        Led::toggle(&mut leds.ld3);
        delay.delay_ms(1000u16);
        Led::toggle(&mut leds.ld3);
        delay.delay_ms(1000u16);

        //explicit on/off
        leds.ld3.on();
        delay.delay_ms(1000u16);
        leds.ld3.off();
        delay.delay_ms(1000u16);
    }
}
