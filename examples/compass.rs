#![no_std]
#![no_main]

extern crate panic_itm;

use cortex_m_rt::{entry, exception};
use cortex_m::iprintln;
use cortex_m::peripheral::syst::SystClkSource;

use stm32f3_discovery::stm32f3xx_hal::prelude::*;
use stm32f3_discovery::stm32f3xx_hal::stm32;

use stm32f3xx_hal::i2c::I2c;

use stm32f3_discovery::switch_hal::{OutputSwitch, ToggleableOutputSwitch};
use stm32f3_discovery::leds::Leds;
use stm32f3_discovery::lsm303dlhc::Lsm303dlhc;

#[entry]
fn main() -> ! {
    let device_periphs = stm32::Peripherals::take().unwrap();
    let mut reset_control_clock = device_periphs.RCC.constrain();

    let mut core_periphs = cortex_m::Peripherals::take().unwrap();
    let mut flash = device_periphs.FLASH.constrain();
    let clocks = reset_control_clock.cfgr.freeze(&mut flash.acr);

    // setup 1 second systick
    let mut syst = core_periphs.SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(8_000_000); // period = 1s
    syst.enable_counter();
    syst.enable_interrupt();

    // initialize user leds
    let gpioe = device_periphs.GPIOE.split(&mut reset_control_clock.ahb);
    let (_leds, _gpioe) = Leds::init(gpioe);

    // setup ITM output
    let stim = &mut core_periphs.ITM.stim[0];

    /*
     * PB6 -> SCL (clock)
     * PB7 -> SDA (data)
     * PE2 -> DRDY (magnometer data ready)
     * PE4 -> INT1 (configurable interrupt 1)
     * PE5 -> INT2 (configurable interrupt 2)
     */
    let mut gpiob = device_periphs.GPIOB.split(&mut reset_control_clock.ahb);
    let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let i2c = I2c::i2c1(device_periphs.I2C1, (scl, sda), 400.khz(), clocks, &mut reset_control_clock.apb1);

    // new lsm303 driver uses continuous mode, so no need wait for interrupts on DRDY
    let mut lsm303dlhc = Lsm303dlhc::new(i2c).unwrap();

    loop {
        let accel = lsm303dlhc.accel().unwrap();
        let mag = lsm303dlhc.mag().unwrap();

        iprintln!(stim, "Accel:{:?}; Mag:{:?}", accel, mag);

        wait_for_interrupt();
    }
}

fn wait_for_interrupt() {
    cortex_m::asm::wfi()
}

#[exception]
fn SysTick() {
    // make sure we don't compile away
    cortex_m::asm::nop();
}