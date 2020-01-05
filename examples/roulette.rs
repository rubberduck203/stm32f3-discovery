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
    let leds = stm32f3_discovery::leds::Leds::init(gpioe);

    let mut compass = [
        leds.ld3,  //N
        leds.ld5,  //NE
        leds.ld7,  //E
        leds.ld9,  //SE
        leds.ld10, //S
        leds.ld8,  //SW
        leds.ld6,  //W
        leds.ld4,  //NW
    ];

    loop {
        //use this syntax instead of ld3.toggle() to disambiuate from ToggleableOutputPin
        //TODO: import just what we need to exposes
        for led in compass.iter_mut() {
            Led::toggle(led);
            delay.delay_ms(200u16);
            Led::toggle(led);
            delay.delay_ms(200u16);
        }
    }
}


