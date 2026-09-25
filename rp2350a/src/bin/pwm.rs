#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::Peri;
use embassy_rp::peripherals::{PIN_4, PIN_15, PWM_SLICE2, PWM_SLICE7};
use embassy_rp::pwm::{Config, Pwm, SetDutyCycle};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};
