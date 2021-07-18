use accelerometer::vector::{F32x3, I16x3};
use accelerometer::{Accelerometer, RawAccelerometer};
use stm32f3xx_hal::gpio;
use stm32f3xx_hal::gpio::{gpiob, OpenDrain};
use stm32f3xx_hal::i2c;
use stm32f3xx_hal::pac;
use stm32f3xx_hal::prelude::*;
use stm32f3xx_hal::rcc;


type Lsm303 =
    lsm303dlhc::Lsm303dlhc<i2c::I2c<pac::I2C1, (gpiob::PB6<gpio::AF4<OpenDrain>>, gpiob::PB7<gpio::AF4<OpenDrain>>)>>;

pub struct Compass {
    lsm303dlhc: Lsm303,
}

impl Compass {
    /// Initialize the onboard Lsm303dhlc e-Compass
    pub fn new<Pb6Mode, Pb7Mode>(
        pb6: gpiob::PB6<Pb6Mode>,
        pb7: gpiob::PB7<Pb7Mode>,
        mode: &mut gpiob::MODER,
        otype: &mut gpiob::OTYPER,
        alternate_function_low: &mut gpiob::AFRL,
        i2c1: pac::I2C1,
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
        let scl = pb6.into_af4_open_drain(mode, otype, alternate_function_low);
        let sda = pb7.into_af4_open_drain(mode, otype, alternate_function_low);
        let i2c = i2c::I2c::new(i2c1, (scl, sda), 400_000.Hz(), clocks, advanced_periph_bus);

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

/// Reads Accelerometer data in G-Force
/// 
/// # Warning
/// This is hard coded for the default settings because the driver doesn't provide a way
/// to read the necessary registers to calculate it based on current settings.
/// If you take control of the underlying device driver and change settings,
/// this will not calculate the correct G-Force values.
impl Accelerometer for Compass {
    type Error = i2c::Error;
    fn accel_norm(&mut self) -> Result<F32x3, accelerometer::Error<Self::Error>> { 
        let reading = self.accel_raw()?;
        /*
         * LA_FS (linear acceleration measurment range [full scale])
         *  can be +/-2, +/-4, +/-8, or +/- 16 
         * LA_So (Linear acceleration sensitivity) can be 1,2,4, or 12
         *  and is measured in milli-G / LSB
         * The driver provides a way to set the range/sensitivy,
         * but no way to read it, so we just hard code the default settings here (for now?).
         *
         * At +/-2g, we get 1mg/LSB (1 mg/bit) resolution.
         * The device returns a 16 bit result.
         * magnitude g / (1 mg / bit) = 
         * +/- 2g range = 4g magnitude
         * 4g / 65535 bits = 4g/(1<<16) = 0.000061 g / bit
         * 2g / 32768 bits = 2g/(1<<15) = 0.000061 g / bit
         * 
         * I _think_ the general equation is:
         * scale_factor = magnitude / #of_bits * sensitivity
         * scale_factor = (abs(range)*2) / #of_bits * sensitivity
         * scale_factor = abs(range) / (#of_bits/2) * sensitivity
         * sf(+/-2g)  =  4g / 65535 bits *  1mg/LSB = 0.000061 g
         * sf(+/-4g)  =  8g / 65535 bits *  2mg/LSB = 0.000244 g
         * sf(+/-8g)  = 16g / 65535 bits *  4mg/LSB = 0.000976 g
         * sf(+/-16g) = 32g / 65535 bits * 12mg/LSB = 0.005859 g
         *
         * NOTE: This also does not account for temperature variance.
         */
        const MAGNITUDE: i32 = 4;
        const NO_OF_BITS: i32 = 1 << 16;
        const SENSITIVITY: i32 = 1;
        const SCALE_FACTOR: f32 = (MAGNITUDE as f32 / NO_OF_BITS as f32) * SENSITIVITY as f32;
        Ok(F32x3::new (
            reading.x as f32 * SCALE_FACTOR,
            reading.y as f32 * SCALE_FACTOR,
            reading.z as f32 * SCALE_FACTOR,
        ))
    }

    fn sample_rate(&mut self) -> Result<f32, accelerometer::Error<<Self as Accelerometer>::Error>> { 
        // we don't expose a way to change this, so hard coded to 400Hz right now
        // it should really be read from the device in case someone snags the raw lsm303dlhc struct,
        // but the driver does't give us a way to read it back from the device
        Ok(400.0)
    }
}
