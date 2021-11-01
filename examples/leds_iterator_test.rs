#![no_std]
#![no_main]

extern crate panic_itm;

use cortex_m_rt::entry;

use stm32f3_discovery::stm32f3xx_hal::delay::Delay;
use stm32f3_discovery::stm32f3xx_hal::pac;
use stm32f3_discovery::stm32f3xx_hal::prelude::*;

use stm32f3_discovery::leds::Leds;
use stm32f3_discovery::switch_hal::OutputSwitch;

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
        let ms_delay = 50u16;

        // iterate through the leds in reverse
        for led in leds.iter_mut().rev() {
            led.on().ok();
            delay.delay_ms(ms_delay);
            led.off().ok();
            delay.delay_ms(ms_delay);
        }

        delay.delay_ms(ms_delay);

        // verify we stop when meeting in the middle
        let mut iter = leds.iter_mut();
        iter.next().map(|led| led.on().ok());
        delay.delay_ms(ms_delay);
        iter.next_back().map(|led| led.on().ok());
        delay.delay_ms(ms_delay);
        iter.next().map(|led| led.on().ok());
        delay.delay_ms(ms_delay);
        iter.next_back().map(|led| led.on().ok());
        delay.delay_ms(ms_delay);
        iter.next().map(|led| led.on().ok());
        delay.delay_ms(ms_delay);
        iter.next_back().map(|led| led.on().ok());
        delay.delay_ms(ms_delay);
        iter.next().map(|led| led.on().ok());
        delay.delay_ms(ms_delay);
        iter.next_back().map(|led| led.on().ok());
        delay.delay_ms(ms_delay);
        // we're in the middle, so panic if either of the next two calls returns a led
        iter.next().map(|_| panic!("Got a Some!"));
        iter.next_back().map(|_| panic!("Got a Some!"));

        // turn everything back off
        for led in &mut leds {
            led.off().ok();
            delay.delay_ms(ms_delay);
        }
    }
}
