//! Provides access to User LEDs LD3-LD10
use stm32f3xx_hal::gpio::gpioe;
use stm32f3xx_hal::gpio::{Output, PushPull};

use switch_hal::{ActiveHigh, IntoSwitch, OutputSwitch, Switch};

/// LED compass direction as noted on the board
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Direction
{
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

pub struct Leds {
    /// North
    pub ld3: Switch<gpioe::PEx<Output<PushPull>>, ActiveHigh>,
    /// NorthWest
    pub ld4: Switch<gpioe::PEx<Output<PushPull>>, ActiveHigh>,
    /// NorthEast
    pub ld5: Switch<gpioe::PEx<Output<PushPull>>, ActiveHigh>,
    /// West
    pub ld6: Switch<gpioe::PEx<Output<PushPull>>, ActiveHigh>,
    /// East
    pub ld7: Switch<gpioe::PEx<Output<PushPull>>, ActiveHigh>,
    /// SouthWest
    pub ld8: Switch<gpioe::PEx<Output<PushPull>>, ActiveHigh>,
    /// SouthEast
    pub ld9: Switch<gpioe::PEx<Output<PushPull>>, ActiveHigh>,
    /// South
    pub ld10: Switch<gpioe::PEx<Output<PushPull>>, ActiveHigh>,
}

impl Leds {
    /// Initializes the user LEDs to OFF
    pub fn new<PE8Mode, PE9Mode, PE10Mode, PE11Mode, PE12Mode, PE13Mode, PE14Mode, PE15Mode>(
        pe8: gpioe::PE8<PE8Mode>,
        pe9: gpioe::PE9<PE9Mode>,
        pe10: gpioe::PE10<PE10Mode>,
        pe11: gpioe::PE11<PE11Mode>,
        pe12: gpioe::PE12<PE12Mode>,
        pe13: gpioe::PE13<PE13Mode>,
        pe14: gpioe::PE14<PE14Mode>,
        pe15: gpioe::PE15<PE15Mode>,
        moder: &mut gpioe::MODER,
        otyper: &mut gpioe::OTYPER,
    ) -> Self {
        let mut leds = Leds {
            ld3: pe9
                .into_push_pull_output(moder, otyper)
                .downgrade()
                .into_active_high_switch(),
            ld4: pe8
                .into_push_pull_output(moder, otyper)
                .downgrade()
                .into_active_high_switch(),
            ld5: pe10
                .into_push_pull_output(moder, otyper)
                .downgrade()
                .into_active_high_switch(),
            ld6: pe15
                .into_push_pull_output(moder, otyper)
                .downgrade()
                .into_active_high_switch(),
            ld7: pe11
                .into_push_pull_output(moder, otyper)
                .downgrade()
                .into_active_high_switch(),
            ld8: pe14
                .into_push_pull_output(moder, otyper)
                .downgrade()
                .into_active_high_switch(),
            ld9: pe12
                .into_push_pull_output(moder, otyper)
                .downgrade()
                .into_active_high_switch(),
            ld10: pe13
                .into_push_pull_output(moder, otyper)
                .downgrade()
                .into_active_high_switch(),
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

        leds
    }

    /// Mutably borrow a LED by the given direction (as noted on the board)
    pub fn for_direction(&mut self, direction: Direction) -> &mut Switch<gpioe::PEx<Output<PushPull>>, ActiveHigh> {
        match direction {
            Direction::North => &mut self.ld3,
            Direction::NorthEast => &mut self.ld5,
            Direction::East => &mut self.ld7,
            Direction::SouthEast => &mut self.ld9,
            Direction::South => &mut self.ld10,
            Direction::SouthWest => &mut self.ld8,
            Direction::West => &mut self.ld6,
            Direction::NorthWest => &mut self.ld4,
        }
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
