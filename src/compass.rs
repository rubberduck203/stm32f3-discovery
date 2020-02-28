use accelerometer::vector::I16x3;
use accelerometer::RawAccelerometer;
use stm32f3xx_hal::gpio;
use stm32f3xx_hal::gpio::gpiob;
use stm32f3xx_hal::i2c;
use stm32f3xx_hal::rcc;
use stm32f3xx_hal::stm32;
use stm32f3xx_hal::time::U32Ext;

type Lsm303 =
    lsm303dlhc::Lsm303dlhc<i2c::I2c<stm32::I2C1, (gpiob::PB6<gpio::AF4>, gpiob::PB7<gpio::AF4>)>>;

pub struct Compass {
    lsm303dlhc: Lsm303,
}

impl Compass {
    /// Initialize the onboard Lsm303dhlc e-Compass
    pub fn new<Pb6Mode, Pb7Mode>(
        pb6: gpiob::PB6<Pb6Mode>,
        pb7: gpiob::PB7<Pb7Mode>,
        mode: &mut gpiob::MODER,
        alternate_function_low: &mut gpiob::AFRL,
        i2c1: stm32::I2C1,
        clocks: rcc::Clocks,
        advanced_periph_bus: &mut rcc::APB1,
    ) -> Result<Self, stm32f3xx_hal::i2c::Error> {
        /*
         * Pinout:
         * PB6 -> SCL (clock)
         * PB7 -> SDA (data)
         * PE2 -> DRDY (magnometer data ready)
         * PE4 -> INT1 (configurable interrupt 1)
         * PE5 -> INT2 (configurable interrupt 2)
         * lsm303hdlc driver uses continuos mode, so no need to wait for interrupts on DRDY
         */
        let scl = pb6.into_af4(mode, alternate_function_low);
        let sda = pb7.into_af4(mode, alternate_function_low);
        let i2c = i2c::I2c::i2c1(i2c1, (scl, sda), 400.khz(), clocks, advanced_periph_bus);

        let lsm303dhlc = Lsm303::new(i2c)?;
        Ok(Compass {
            lsm303dlhc: lsm303dhlc,
        })
    }

    /// Read the raw magnetometer data
    pub fn mag_raw(&mut self) -> Result<I16x3, i2c::Error> {
        let reading = self.lsm303dlhc.mag()?;
        Ok(I16x3::new(reading.x, reading.y, reading.z))
    }

    /// Consume the Compass and return the underlying Lsm303dhlc
    pub fn into_lsm303dlhc(self) -> Lsm303 {
        self.lsm303dlhc
    }
}

impl RawAccelerometer<I16x3> for Compass {
    type Error = i2c::Error;

    /// Read the raw accelerometer data
    fn accel_raw(&mut self) -> Result<I16x3, accelerometer::Error<Self::Error>> {
        let reading = self.lsm303dlhc.accel()?;
        Ok(I16x3::new(reading.x, reading.y, reading.z))
    }
}
