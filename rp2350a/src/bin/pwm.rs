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

#[embassy_executor::task]
async fn pwm_set_config(slice7: Peri<'static, PWM_SLICE7>, pin15: Peri<'static, PIN_15>) {
    let mut c = Config::default();
    c.top = 32_768;
    c.compare_b = 8;
    let mut pwm = Pwm::new_output_b(slice7, pin15, c.clone());

    loop {
        info!("current LED duty cycle: {}/32768", c.compare_b);
        Timer::after_secs(1).await;
        c.compare_b = c.compare_b.rotate_left(4);
        pwm.set_config(&c);
    }
}

#[embassy_executor::task]
async fn pwm_set_dutycycle(slice2: Peri<'static, PWM_SLICE2>, pin4: Peri<'static, PIN_4>) {
    let desired_freq_hz = 25_000;
    let clock_freq_hz = embassy_rp::clocks::clk_sys_freq();
    let divider = 16u8;
    let period = (clock_freq_hz / (desired_freq_hz * divider as u32)) as u16 - 1;

    let mut c = Config::default();
    c.top = period;
    c.divider = divider.into();

    let mut pwm = Pwm::new_output_a(slice2, pin4, c.clone());

    loop {
        pwm.set_duty_cycle_fully_on().unwrap();
        Timer::after_secs(1).await;

        pwm.set_duty_cycle_percent(66).unwrap();
        Timer::after_secs(1).await;

        pwm.set_duty_cycle(c.top / 4).unwrap();
        Timer::after_secs(1).await;

        pwm.set_duty_cycle_fully_off().unwrap();
        Timer::after_secs(1).await;
    }
}
