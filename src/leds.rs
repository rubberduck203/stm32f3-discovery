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
    pub ld6: gpioe::PEx<Output<PushPull>>,
    pub ld7: gpioe::PEx<Output<PushPull>>,
    pub ld8: gpioe::PEx<Output<PushPull>>,
    pub ld9: gpioe::PEx<Output<PushPull>>,
    pub ld10: gpioe::PEx<Output<PushPull>>,
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
            ld6: gpioe
                 .pe15
                 .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper)
                 .downgrade(),
            ld7: gpioe
                 .pe11
                 .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper)
                 .downgrade(),
            ld8: gpioe
                 .pe14
                 .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper)
                 .downgrade(),
            ld9: gpioe
                 .pe12
                 .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper)
                 .downgrade(),
            ld10: gpioe
                 .pe13
                 .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper)
                 .downgrade(),
        };

        //TODO: expose an iterator
        leds.ld3.off();
        leds.ld4.off();
        leds.ld5.off();
        leds.ld6.off();
        leds.ld7.off();
        leds.ld8.off();
        leds.ld9.off();
        leds.ld10.off();

        leds
    }
}
