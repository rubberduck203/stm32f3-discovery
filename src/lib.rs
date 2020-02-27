#![no_std]

pub use lsm303dlhc;
pub use stm32f3xx_hal;
pub use switch_hal;

pub mod button;
pub mod leds;

/// Signals the process to go into low power mode until an interrupt occurs
pub fn wait_for_interrupt() {
    cortex_m::asm::wfi()
}

pub mod compass {
    use stm32f3xx_hal::gpio;
    use stm32f3xx_hal::gpio::gpiob;
    use stm32f3xx_hal::i2c;
    use stm32f3xx_hal::rcc;
    use stm32f3xx_hal::stm32;
    use stm32f3xx_hal::time::U32Ext;

    type Lsm303 = lsm303dlhc::Lsm303dlhc<
        i2c::I2c<stm32::I2C1, (gpiob::PB6<gpio::AF4>, gpiob::PB7<gpio::AF4>)>,
    >;

    pub struct Compass {
        //todo: encapsulate
        pub lsm303dlhc: Lsm303
    }

    impl Compass {
        pub fn new<Pb6Mode, Pb7Mode>(
            pb6: gpiob::PB6<Pb6Mode>,
            pb7: gpiob::PB7<Pb7Mode>,
            mode: &mut gpiob::MODER,
            alternate_function_low: &mut gpiob::AFRL,
            i2c1: stm32::I2C1,
            clocks: rcc::Clocks,
            advanced_periph_bus: &mut rcc::APB1,
        ) -> Result<Self, stm32f3xx_hal::i2c::Error> {
            let scl = pb6.into_af4(mode, alternate_function_low);
            let sda = pb7.into_af4(mode, alternate_function_low);
            let i2c = i2c::I2c::i2c1(i2c1, (scl, sda), 400.khz(), clocks, advanced_periph_bus);

            let lsm303dhlc = Lsm303::new(i2c)?;
            Ok(Compass {lsm303dlhc: lsm303dhlc})
        }
    }
}
