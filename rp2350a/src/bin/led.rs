#![no_std]
#![no_main]

use cyw43::aligned_bytes;
use cyw43_pio::{PioSpi, RM2_CLOCK_DIVIDER};
use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());
    let (_net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw, nvram).await;
    spawner.spawn(unwrap!(cyw43_task(runner)));
    let delay = Duration::from_millis(250);
    loop {
        info!("led on!");
        control.gpio_set(0, true).await;
