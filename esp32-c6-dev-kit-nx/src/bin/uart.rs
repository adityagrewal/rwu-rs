#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embedded_io_async::{Read, Write};
use panic_rtt_target as _;
use esp_hal::{timer::timg::TimerGroup, uart::{Config as UartConfig, Uart}};
use esp_hal::interrupt::software::SoftwareInterruptControl;

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(_spawner: Spawner) {
    rtt_target::rtt_init_defmt!();
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let software_interrupt = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, software_interrupt.software_interrupt0);

    let mut uart = Uart::new(peripherals.UART1, UartConfig::default())
        .unwrap()
        .with_rx(peripherals.GPIO1)
        .with_tx(peripherals.GPIO0)
        .into_async();

    let mut buf = [0u8; 1];
    loop {
        Read::read(&mut uart, &mut buf).await.unwrap();
        Write::write_all(&mut uart, &buf).await.unwrap();
    }
}
