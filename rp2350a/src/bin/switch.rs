#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_15, Level::Low);
    let mut button = Input::new(p.PIN_16, Pull::Up);

    loop {
        button.wait_for_any_edge().await;
        Timer::after_millis(20).await; // settle before sampling
        // button is active low: pressed pulls the pin down
        led.set_level(if button.is_low() { Level::High } else { Level::Low });
    }
}
