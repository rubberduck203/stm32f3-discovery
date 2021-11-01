#![no_std]
#![no_main]

extern crate panic_itm;

use cortex_m_rt::entry;

use stm32f3_discovery::stm32f3xx_hal::delay::Delay;
use stm32f3_discovery::stm32f3xx_hal::pac;
use stm32f3_discovery::stm32f3xx_hal::prelude::*;

use stm32f3xx_hal::gpio::gpioe;
use stm32f3xx_hal::gpio::{Output, PushPull};

use stm32f3_discovery::leds::{Direction, Leds};
use stm32f3_discovery::switch_hal::{ActiveHigh, OutputSwitch, Switch};

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

    loop {
        let fast_delay = 50u16;
        for direction in Direction::iter() {
            let led = &mut leds.for_direction(*direction);

            led.on().ok();
            delay.delay_ms(fast_delay);
            led.off().ok();
            delay.delay_ms(fast_delay);
        }

        slow_blink(leds.for_direction(Direction::North), &mut delay);
        slow_blink(leds.for_direction(Direction::South), &mut delay);
        slow_blink(leds.for_direction(Direction::East), &mut delay);
        slow_blink(leds.for_direction(Direction::West), &mut delay);
    }
}

fn slow_blink(switch: &mut Switch<gpioe::PEx<Output<PushPull>>, ActiveHigh>, delay: &mut Delay) {
    let slow_delay = 250u16;
    switch.on().ok();
    delay.delay_ms(slow_delay);
    switch.off().ok();
    delay.delay_ms(slow_delay);
}
