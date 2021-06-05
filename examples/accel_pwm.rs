//! The three LEDs should gradually light up corresponding to the orientation of the board along its three axes
//! (or more precisely they indicate the direction of Earth's gravity relative to the three axes of the board):
//!
//!  - Lying down flat the blue LDE should be full on, indicating that gravity is perpendicular to the board.
//!  - @hen you slowly turn it around its long axis the green LED should gradually go on while the blue gradually fades out.
//!    The green LED should be fully on when the green LED is 'facing down' and the other axes are straight.
//!  - Likewise when you turn it along the short axis (from a flat position) the red LED will go on.

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
use stm32f3xx_hal::{prelude::*, pac};

#[entry]
fn main() -> ! {
    let device_periphs = pac::Peripherals::take().unwrap();
    let mut reset_and_clock_control = device_periphs.RCC.constrain();
    let core_periphs = cortex_m::Peripherals::take().unwrap();
    // Configure our clocks
    let mut flash = device_periphs.FLASH.constrain();

    let clocks = reset_and_clock_control.cfgr.freeze(&mut flash.acr);
    let mut delay = Delay::new(core_periphs.SYST, clocks);
    let mut gpiob = device_periphs.GPIOB.split(&mut reset_and_clock_control.ahb);

    // Prep the pins we need in their correct alternate function
    let mut gpioe = device_periphs.GPIOE.split(&mut reset_and_clock_control.ahb);

    let led_blue = gpioe.pe8.into_af2_push_pull(&mut gpioe.moder, &mut gpioe.otyper, &mut gpioe.afrh);
    let led_green = gpioe.pe11.into_af2_push_pull(&mut gpioe.moder, &mut gpioe.otyper, &mut gpioe.afrh);
    let lef_red = gpioe.pe13.into_af2_push_pull(&mut gpioe.moder, &mut gpioe.otyper, &mut gpioe.afrh);

    let max_duty = 4096;

    let tim1_channels = tim1(
        device_periphs.TIM1,
        max_duty, // resolution of duty cycle
        500.Hz(), // frequency of period
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
        &mut gpiob.otyper,
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

        // probably the accel vector should be transformed to angles, but the steeper-than-usual slope
        // used in LED_RAMP seems to have the same effect.

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

// inverse power-law ramp adapted from
// https://forum.arduino.cc/index.php?topic=147818.msg1113233#msg1113233

// float a = 0.25; // usually either 0.5 or 0.33, but 0.25 looks best in this case
// int Pmax = pow(4096,a);
// int N = 255;
// for (int n = 0; n <= N; ++n)
// {
//         std::cout << int(pow(Pmax * ((float)n/(float)N), 1/a) + 0.5) << ", ";
// }
// TODO: generate with macro?
static LED_RAMP: [u16; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 10, 10, 11, 12, 13, 13,
    14, 15, 16, 17, 18, 20, 21, 22, 23, 25, 26, 28, 29, 31, 32, 34, 36, 38, 40, 42, 44, 46, 48, 51,
    53, 55, 58, 61, 64, 66, 69, 72, 76, 79, 82, 86, 89, 93, 97, 101, 105, 109, 113, 118, 122, 127,
    132, 137, 142, 147, 152, 158, 164, 169, 175, 182, 188, 194, 201, 208, 215, 222, 229, 237, 244,
    252, 260, 268, 277, 285, 294, 303, 312, 322, 331, 341, 351, 362, 372, 383, 394, 405, 417, 428,
    440, 452, 465, 477, 490, 504, 517, 531, 545, 559, 574, 589, 604, 619, 635, 651, 667, 684, 701,
    718, 736, 753, 772, 790, 809, 828, 848, 868, 888, 909, 930, 951, 972, 995, 1017, 1040, 1063,
    1086, 1110, 1135, 1159, 1185, 1210, 1236, 1262, 1289, 1316, 1344, 1372, 1401, 1430, 1459, 1489,
    1519, 1550, 1581, 1613, 1645, 1678, 1711, 1744, 1779, 1813, 1848, 1884, 1920, 1957, 1994, 2032,
    2070, 2109, 2148, 2188, 2228, 2269, 2311, 2353, 2396, 2439, 2483, 2527, 2572, 2618, 2664, 2711,
    2758, 2806, 2855, 2904, 2954, 3005, 3056, 3108, 3161, 3214, 3268, 3322, 3378, 3434, 3490, 3548,
    3606, 3664, 3724, 3784, 3845, 3907, 3969, 4032, 4096,
];

fn led_ramp(v: f32) -> u16 {
    let v = if v < 0.0 {
        0.0
    } else if v > 1.0 {
        1.0
    } else {
        v
    };
    LED_RAMP[(v * 255.0) as usize]
}
