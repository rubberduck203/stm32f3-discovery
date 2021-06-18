#![no_std]
#![no_main]

extern crate panic_itm;

use cortex_m_rt::entry;

use stm32f3_discovery::stm32f3xx_hal::delay::Delay;
use stm32f3_discovery::stm32f3xx_hal::prelude::*;
use stm32f3_discovery::stm32f3xx_hal::pac;

use stm32f3_discovery::leds::{Direction, Leds};
use stm32f3_discovery::switch_hal::OutputSwitch;

fn one_round_by_direction(leds: &mut Leds, delay: &mut Delay) {
    let slow_delay = 500u16;
    let directions = [
        Direction::North,
        Direction::NorthEast,
        Direction::East,
        Direction::SouthEast,
        Direction::South,
        Direction::SouthWest,
        Direction::West,
        Direction::NorthWest
    ];

    for direction in &directions {
        let led = &mut leds.for_direction(*direction);

        led.on().ok();
        delay.delay_ms(slow_delay);
        led.off().ok();
        delay.delay_ms(slow_delay);
    }
}

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
    let mut leds = Leds::new(
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

    // Light them one round by direction.
    one_round_by_direction(&mut leds, &mut delay);

    // Finally let go by the compass array.
    let mut compass = leds.into_array();

    loop {
        let ms_delay = 50u16;
        let ubound = compass.len();
        for curr in 0..ubound {
            let next = (curr + 1) % ubound;

            compass[next].on().ok();
            delay.delay_ms(ms_delay);
            compass[curr].off().ok();
            delay.delay_ms(ms_delay);
        }

        // Alternative way to iterate through lights
        // for led in compass.iter_mut() {
        //     led.on().ok();
        //     delay.delay_ms(ms_delay);
        //     led.off().ok();
        //     delay.delay_ms(ms_delay);
        // }
    }
}
