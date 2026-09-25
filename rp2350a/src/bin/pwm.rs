#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::Peri;
use embassy_rp::peripherals::{PIN_4, PIN_15, PWM_SLICE2, PWM_SLICE7};
use embassy_rp::pwm::{Config, Pwm, SetDutyCycle};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    spawner.spawn(pwm_set_config(p.PWM_SLICE7, p.PIN_15).unwrap());
    spawner.spawn(pwm_set_dutycycle(p.PWM_SLICE2, p.PIN_4).unwrap());
}
