use stm32f3xx_hal::hal::digital::v2::{OutputPin, ToggleableOutputPin};
use stm32f3xx_hal::gpio::{Output, PushPull};
use stm32f3xx_hal::gpio::gpioe;

pub trait Led {
    fn on(&mut self);
    fn off(&mut self);
    fn toggle(&mut self);
}

impl Led for gpioe::PEx<Output<PushPull>> {
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
    pub ld3: gpioe::PEx<Output<PushPull>>,
    pub ld4: gpioe::PEx<Output<PushPull>>,
    pub ld5: gpioe::PEx<Output<PushPull>>,
}

impl Leds {
    // TODO: this takes ownership of the entire gpioe port
    // This needs to erase the pins we've taken control of
    // and return a new gpioe with the remaining available pins
    // init(mut gpioe: Parts) -> (Self, NewGpioE)
    pub fn init(mut gpioe: gpioe::Parts) -> Self {
        let mut leds = Leds {
            ld3: gpioe
                .pe9
                .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper)
                .downgrade(),
            ld4: gpioe
                 .pe8
                 .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper)
                 .downgrade(),
            ld5: gpioe
                 .pe10
                 .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper)
                 .downgrade(),
        };

        leds.ld3.off();
        leds.ld4.off();
        leds.ld5.off();

        leds
    }
}
