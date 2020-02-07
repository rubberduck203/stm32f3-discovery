use stm32f3xx_hal::gpio::gpioe;
use stm32f3xx_hal::gpio::{Floating, Input, Output, PushPull};

use switch_hal::{ActiveHigh, Switch, OutputSwitch};

/// GpioE after Led pins (PE8-PE15) have been moved
/// If you intend to use those pins for other functions, DO NOT call Leds::init().
/// You'll have to initialize the pins yourself.
pub struct GpioE {
    /// Opaque AFRH register
    pub afrh: gpioe::AFRH,
    /// Opaque AFRL register
    pub afrl: gpioe::AFRL,
    /// Opaque MODER register
    pub moder: gpioe::MODER,
    /// Opaque OTYPER register
    pub otyper: gpioe::OTYPER,
    /// Opaque PUPDR register
    pub pupdr: gpioe::PUPDR,

    pub pe0: gpioe::PE0<Input<Floating>>,
    pub pe1: gpioe::PE1<Input<Floating>>,
    pub pe2: gpioe::PE2<Input<Floating>>,
    pub pe3: gpioe::PE3<Input<Floating>>,
    pub pe4: gpioe::PE4<Input<Floating>>,
    pub pe5: gpioe::PE5<Input<Floating>>,
    pub pe6: gpioe::PE6<Input<Floating>>,
    pub pe7: gpioe::PE7<Input<Floating>>,
}

pub struct Leds {
    pub ld3: Switch<gpioe::PEx<Output<PushPull>>, ActiveHigh>,
    pub ld4: Switch<gpioe::PEx<Output<PushPull>>, ActiveHigh>,
    pub ld5: Switch<gpioe::PEx<Output<PushPull>>, ActiveHigh>,
    pub ld6: Switch<gpioe::PEx<Output<PushPull>>, ActiveHigh>,
    pub ld7: Switch<gpioe::PEx<Output<PushPull>>, ActiveHigh>,
    pub ld8: Switch<gpioe::PEx<Output<PushPull>>, ActiveHigh>,
    pub ld9: Switch<gpioe::PEx<Output<PushPull>>, ActiveHigh>,
    pub ld10: Switch<gpioe::PEx<Output<PushPull>>, ActiveHigh>,
}

impl Leds {
    /// Initializes the user LEDs
    ///
    /// ## Returns
    /// A tuple of `(Leds, GpioE)`, where `Leds` has taken ownership of PE8-PE15
    /// and `GpioE` contains the remaining members of `stm32f3xx_hal::gpio::GPIOE`
    ///
    /// **If you intend to use those pins for other functions, DO NOT call Leds::init().**
    /// You'll have to initialize the pins yourself.
    pub fn init(mut gpioe: gpioe::Parts) -> (Self, GpioE) {
        let mut leds = Leds {
            ld3: Switch::<_, ActiveHigh>::new(
                gpioe
                    .pe9
                    .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper)
                    .downgrade(),
            ),
            ld4: Switch::<_, ActiveHigh>::new(
                gpioe
                    .pe8
                    .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper)
                    .downgrade(),
            ),
            ld5: Switch::<_, ActiveHigh>::new(
                gpioe
                    .pe10
                    .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper)
                    .downgrade(),
            ),
            ld6: Switch::<_, ActiveHigh>::new(
                gpioe
                    .pe15
                    .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper)
                    .downgrade(),
            ),
            ld7: Switch::<_, ActiveHigh>::new(
                gpioe
                    .pe11
                    .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper)
                    .downgrade(),
            ),
            ld8: Switch::<_, ActiveHigh>::new(
                gpioe
                    .pe14
                    .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper)
                    .downgrade(),
            ),
            ld9: Switch::<_, ActiveHigh>::new(
                gpioe
                    .pe12
                    .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper)
                    .downgrade(),
            ),
            ld10: Switch::<_, ActiveHigh>::new(
                gpioe
                    .pe13
                    .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper)
                    .downgrade(),
            ),
        };

        //TODO: expose an iterator
        leds.ld3.off().ok();
        leds.ld4.off().ok();
        leds.ld5.off().ok();
        leds.ld6.off().ok();
        leds.ld7.off().ok();
        leds.ld8.off().ok();
        leds.ld9.off().ok();
        leds.ld10.off().ok();

        (
            leds,
            GpioE {
                afrh: gpioe.afrh,
                afrl: gpioe.afrl,
                moder: gpioe.moder,
                otyper: gpioe.otyper,
                pupdr: gpioe.pupdr,
                pe0: gpioe.pe0,
                pe1: gpioe.pe1,
                pe2: gpioe.pe2,
                pe3: gpioe.pe3,
                pe4: gpioe.pe4,
                pe5: gpioe.pe5,
                pe6: gpioe.pe6,
                pe7: gpioe.pe7,
            },
        )
    }

    /// Consumes the `Leds` struct and returns an array
    /// where index 0 is N and each incrementing index
    /// rotates clockwise around the compass
    pub fn into_array(self) -> [Switch<gpioe::PEx<Output<PushPull>>, ActiveHigh>; 8] {
        [
            self.ld3,  //N
            self.ld5,  //NE
            self.ld7,  //E
            self.ld9,  //SE
            self.ld10, //S
            self.ld8,  //SW
            self.ld6,  //W
            self.ld4,  //NW
        ]
    }
}
