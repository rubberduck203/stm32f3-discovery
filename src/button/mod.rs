pub mod hal;
pub mod interrupt;

use hal::{ActiveHighButton, Button};
use stm32f3xx_hal::gpio::gpioa::PA0;
use stm32f3xx_hal::gpio::{Floating, Input};

/// Wrapper struct around `ActiveHighButton<PA0<Input<Floating>>>`
/// It's floating because there's an external pull down resistor and low pass filter circuit.
pub struct UserButton(ActiveHighButton<PA0<Input<Floating>>>);

impl UserButton {
    /// Typesafe constructor for the UserButton peripheral on PA0.
    /// It's impossible to construct this button with the wrong pin or pin state.
    /// It's also impossible to construct more than one `UserButton` instance because `gpioa.pa0` is moved upon construction.
    pub fn new(pa0: PA0<Input<Floating>>) -> Self {
        UserButton(ActiveHighButton::new(pa0))
    }
}

impl Button for UserButton {
    type Error = ();
    fn is_pressed(&self) -> Result<bool, Self::Error> {
        self.0.is_pressed()
    }
}
