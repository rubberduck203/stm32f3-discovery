#![deny(unsafe_code)]
#![no_std]
#![no_main]

extern crate panic_itm;

use cortex_m_rt::entry;
use stm32f3_discovery::accelerometer::RawAccelerometer;
use stm32f3_discovery::compass::Compass;
use stm32f3_discovery::stm32f3xx_hal;
use stm32f3xx_hal::delay::Delay;
use stm32f3xx_hal::pwm::tim1;
use stm32f3xx_hal::{prelude::*, stm32};

#[entry]
fn main() -> ! {
    let device_periphs = stm32::Peripherals::take().unwrap();
    let mut reset_and_clock_control = device_periphs.RCC.constrain();
    let core_periphs = cortex_m::Peripherals::take().unwrap();
    // Configure our clocks
    let mut flash = device_periphs.FLASH.constrain();

    let clocks = reset_and_clock_control.cfgr.freeze(&mut flash.acr);
    let mut delay = Delay::new(core_periphs.SYST, clocks);
    let mut gpiob = device_periphs.GPIOB.split(&mut reset_and_clock_control.ahb);

    // Prep the pins we need in their correct alternate function
    let mut gpioe = device_periphs.GPIOE.split(&mut reset_and_clock_control.ahb);

    let led_blue = gpioe.pe8.into_af2(&mut gpioe.moder, &mut gpioe.afrh);
    let led_green = gpioe.pe11.into_af2(&mut gpioe.moder, &mut gpioe.afrh);
    let lef_red = gpioe.pe13.into_af2(&mut gpioe.moder, &mut gpioe.afrh);

    let max_duty = 1280;

    let tim1_channels = tim1(
        device_periphs.TIM1,
        max_duty, // resolution of duty cycle
        500.hz(), // frequency of period
        &clocks,  // To get the timer's clock speed
    );

    let mut pwm_ch_blue = tim1_channels.0.output_to_pe8(led_blue);
    pwm_ch_blue.set_duty(0);
    pwm_ch_blue.enable();

    let mut pwm_ch_green = tim1_channels.1.output_to_pe11(led_green);
    pwm_ch_green.set_duty(0);
    pwm_ch_green.enable();

    let mut pwm_ch_red = tim1_channels.2.output_to_pe13(lef_red);
    pwm_ch_red.set_duty(0);
    pwm_ch_red.enable();

    let mut compass = Compass::new(
        gpiob.pb6,
        gpiob.pb7,
        &mut gpiob.moder,
        &mut gpiob.afrl,
        device_periphs.I2C1,
        clocks,
        &mut reset_and_clock_control.apb1,
    )
    .unwrap();

    loop {
        const SENSITIVITY: f32 = 1. / (1 << 14) as f32;

        let acc = compass.accel_raw().unwrap();

        let x = f32_abs(f32::from(acc.x) * SENSITIVITY);
        let y = f32_abs(f32::from(acc.y) * SENSITIVITY);
        let z = f32_abs(f32::from(acc.z) * SENSITIVITY);

        let r = led_ramp(x);
        let g = led_ramp(y);
        let b = led_ramp(z);

        pwm_ch_blue.set_duty(b);
        pwm_ch_green.set_duty(g);
        pwm_ch_red.set_duty(r);
        delay.delay_ms(100_u16);
    }
}

// FIXME: can we use f32::abs somehow?
fn f32_abs(v: f32) -> f32 {
    if v < 0.0 {
        -v
    } else {
        v
    }
}

fn led_ramp(v: f32) -> u16 {
    let v = if v < 0.0 {
        0.0
    } else if v > 1.0 {
        1.0
    } else {
        v
    };
    // inverse power-law ramp from
    // https://forum.arduino.cc/index.php?topic=147818.msg1113233#msg1113233
    [
        0, 0, 0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 3, 4, 4, 5, 5, 6, 7, 7, 8, 9, 10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 22, 23, 24, 26, 27, 28, 30, 31, 33, 35, 36, 38, 40, 41, 43, 45, 47, 49,
        51, 53, 55, 57, 59, 61, 63, 65, 67, 70, 72, 74, 77, 79, 81, 84, 86, 89, 92, 94, 97, 100,
        102, 105, 108, 111, 114, 117, 120, 123, 126, 129, 132, 135, 138, 141, 145, 148, 151, 155,
        158, 162, 165, 169, 172, 176, 180, 183, 187, 191, 194, 198, 202, 206, 210, 214, 218, 222,
        226, 230, 234, 239, 243, 247, 252, 256, 260, 265, 269, 274, 278, 283, 287, 292, 297, 301,
        306, 311, 316, 321, 326, 331, 336, 341, 346, 351, 356, 361, 366, 372, 377, 382, 388, 393,
        398, 404, 409, 415, 421, 426, 432, 438, 443, 449, 455, 461, 467, 473, 479, 485, 491, 497,
        503, 509, 515, 521, 528, 534, 540, 547, 553, 559, 566, 572, 579, 586, 592, 599, 606, 612,
        619, 626, 633, 640, 647, 654, 661, 668, 675, 682, 689, 696, 703, 711, 718, 725, 733, 740,
        748, 755, 763, 770, 778, 786, 793, 801, 809, 816, 824, 832, 840, 848, 856, 864, 872, 880,
        888, 896, 905, 913, 921, 930, 938, 946, 955, 963, 972, 980, 989, 997, 1006, 1015, 1024,
        1032, 1041, 1050, 1059, 1068, 1077, 1086, 1095, 1104, 1113, 1122, 1131, 1140, 1150, 1159,
        1168, 1178, 1187, 1196, 1206, 1215, 1280,
    ][(v * 255.0) as usize]
}
