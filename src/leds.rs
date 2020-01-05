use stm32f3xx_hal::hal::digital::v2::{OutputPin, ToggleableOutputPin};
use stm32f3xx_hal::gpio::{Output, PushPull};
use stm32f3xx_hal::gpio::gpioe::*;

pub trait Led {
    fn on(&mut self);
    fn off(&mut self);
    fn toggle(&mut self);
}

//TODO: Probably use a macro to generate impls for the right pins
impl Led for PE9<Output<PushPull>> {
    fn on(&mut self) {
        self.set_high().unwrap();
    }
    
    fn off(&mut self) {
        self.set_low().unwrap();
    }
    
    fn toggle(&mut self) {
        ToggleableOutputPin::toggle(self).unwrap();
    }
}

pub struct Leds {
    pub ld3: PE9<Output<PushPull>>,
}

impl Leds {
    // TODO: this takes ownership of the entire gpioe port
    // This needs to erase the pins we've taken control of
    // and return a new gpioe with the remaining available pins
    // init(mut gpioe: Parts) -> (Self, NewGpioE)
    pub fn init(mut gpioe: Parts) -> Self {
        Leds {
            ld3: gpioe
                .pe9
                .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper),
        }
    }
}
