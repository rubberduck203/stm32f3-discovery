//! Provides access to the user button on PA0
pub mod interrupt;

use stm32f3xx_hal::gpio::gpioa::PA0;
use stm32f3xx_hal::gpio::{gpioa, Input};
use switch_hal::{ActiveHigh, InputSwitch, IntoSwitch, Switch};

/// Wrapper struct around `ActiveHighButton<PA0<Input>>`
/// The user button has an external pull down resistor and low pass filter circuit.
pub struct UserButton(Switch<PA0<Input>, ActiveHigh>);

impl UserButton {
    /// Typesafe constructor for the user button peripheral on PA0.
    /// It's impossible to construct this button with the wrong pin or pin state.
    /// It's also impossible to construct more than one `UserButton` instance because `gpioa.pa0` is moved upon construction.
    pub fn new(pa0: PA0<Input>, moder: &mut gpioa::MODER, pupdr: &mut gpioa::PUPDR) -> Self {
        // This button is equipped with an external pull-down and so there is
        // no need to use the internal one.
        UserButton(
            pa0.into_floating_input(moder, pupdr)
                .into_active_high_switch(),
        )
    }
}

impl InputSwitch for UserButton {
    type Error = core::convert::Infallible;
    fn is_active(&self) -> Result<bool, Self::Error> {
        self.0.is_active()
    }
}
