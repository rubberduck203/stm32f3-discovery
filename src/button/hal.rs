use stm32f3xx_hal::hal::digital::v2::InputPin;

pub trait Button {
    type Error;
    fn is_pressed(&self) -> Result<bool, Self::Error>;
}

pub struct ActiveHighButton<T>
where
    T: InputPin,
{
    pin: T,
}

impl<T: InputPin> ActiveHighButton<T> {
    pub fn new(pin: T) -> Self {
        ActiveHighButton { pin: pin }
    }
}

impl<T: InputPin> Button for ActiveHighButton<T> {
    type Error = <T as stm32f3xx_hal::hal::digital::v2::InputPin>::Error;
    fn is_pressed(&self) -> Result<bool, Self::Error> {
        self.pin.is_high()
    }
}

pub struct ActiveLowButton<T>
where
    T: InputPin,
{
    pin: T,
}

impl<T: InputPin> ActiveLowButton<T> {
    pub fn new(pin: T) -> Self {
        ActiveLowButton { pin: pin }
    }
}

impl<T: InputPin> Button for ActiveLowButton<T> {
    type Error = <T as stm32f3xx_hal::hal::digital::v2::InputPin>::Error;
    fn is_pressed(&self) -> Result<bool, Self::Error> {
        self.pin.is_low()
    }
}
