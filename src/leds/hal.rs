use stm32f3xx_hal::hal::digital::v2::{OutputPin, ToggleableOutputPin};

pub trait Led {
    type Error;

    fn on(&mut self) -> Result<(), Self::Error>;
    fn off(&mut self) -> Result<(), Self::Error>;
}

pub trait ToggleableLed {
    type Error;

    fn toggle(&mut self) -> Result<(), Self::Error>;
}

pub struct ActiveHighLed<T>
where
    T: OutputPin,
{
    pin: T,
}

impl<T: OutputPin> ActiveHighLed<T> {
    pub fn new(pin: T) -> Self {
        ActiveHighLed { pin: pin }
    }
}

impl<T: OutputPin> Led for ActiveHighLed<T> {
    type Error = <T as OutputPin>::Error;

    fn on(&mut self) -> Result<(), Self::Error> {
        self.pin.set_high()
    }

    fn off(&mut self) -> Result<(), Self::Error> {
        self.pin.set_low()
    }
}

impl<T: OutputPin + ToggleableOutputPin> ToggleableLed for ActiveHighLed<T> {
    type Error = <T as ToggleableOutputPin>::Error;

    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.pin.toggle()
    }
}

pub struct ActiveLowLed<T>
where
    T: OutputPin,
{
    pin: T,
}

impl<T: OutputPin> ActiveLowLed<T> {
    pub fn new(pin: T) -> Self {
        ActiveLowLed { pin: pin }
    }
}

impl<T: OutputPin> Led for ActiveLowLed<T> {
    type Error = <T as OutputPin>::Error;

    fn on(&mut self) -> Result<(), Self::Error> {
        self.pin.set_low()
    }

    fn off(&mut self) -> Result<(), Self::Error> {
        self.pin.set_high()
    }
}

impl<T: OutputPin + ToggleableOutputPin> ToggleableLed for ActiveLowLed<T> {
    type Error = <T as ToggleableOutputPin>::Error;

    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.pin.toggle()
    }
}