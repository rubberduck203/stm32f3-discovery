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
        let ms_delay = 50u16;
        let ubound = compass.len();
        for curr in 0..ubound {
            let next = (curr + 1) % ubound;

            compass[next].on();
            delay.delay_ms(ms_delay);
            compass[curr].off();
            delay.delay_ms(ms_delay);
        }

        // Alternative way to iterate through lights
        // for led in compass.iter_mut() {
        //     led.on();
        //     delay.delay_ms(ms_delay);
        //     led.off();
        //     delay.delay_ms(ms_delay);
        // }
    }
}
