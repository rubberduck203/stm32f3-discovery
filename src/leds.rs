use stm32f3xx_hal::hal::digital::v2::{OutputPin, ToggleableOutputPin};
use stm32f3xx_hal::gpio::{Output, PushPull};
use stm32f3xx_hal::gpio::gpioe;

pub trait Led {
    fn on(&mut self);
    fn off(&mut self);
    fn toggle(&mut self);
}

macro_rules! led {
    ($pin:ident) => {
        impl Led for gpioe::$pin<Output<PushPull>> {
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
    };
}

led!(PE9);
led!(PE8);
led!(PE10);

pub struct Leds {
    pub ld3: gpioe::PE9<Output<PushPull>>,
    pub ld4: gpioe::PE8<Output<PushPull>>,
    pub ld5: gpioe::PE10<Output<PushPull>>,
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
                .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper),
            ld4: gpioe
                 .pe8
                 .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper),
            ld5: gpioe
                 .pe10
                 .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper),
        };

        leds.ld3.off();
        leds.ld4.off();
        leds.ld5.off();

        leds
    }
}
