#![no_std]
#![no_main]

use panic_rtt_target as _;
use esp_hal::{
    delay::Delay,
    gpio::DriveMode,
    ledc::{channel, channel::ChannelIFace, timer, timer::TimerIFace, LSGlobalClkSource, Ledc, LowSpeed},
    main,
    time::Rate,
};

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    rtt_target::rtt_init_defmt!();
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut ledc = Ledc::new(peripherals.LEDC);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

    let mut lstimer0 = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    lstimer0
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty14Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: Rate::from_khz(1),
        })
        .unwrap();

    let mut channel0 = ledc.channel(channel::Number::Channel0, peripherals.GPIO3);
    channel0
        .configure(channel::config::Config {
            timer: &lstimer0,
            duty_pct: 0,
            drive_mode: DriveMode::PushPull,
        })
        .unwrap();

    let delay = Delay::new();
    let mut duty: u8 = 0;
    let mut rising = true;
    loop {
        channel0.set_duty(duty).unwrap();
        if rising {
            duty += 1;
            if duty == 100 { rising = false; }
        } else {
            duty -= 1;
            if duty == 0 { rising = true; }
        }
        delay.delay_millis(20);
    }
}
