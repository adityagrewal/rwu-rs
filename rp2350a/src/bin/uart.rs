#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::dma;
use embassy_rp::peripherals::{DMA_CH0, DMA_CH1, UART0};
use embassy_rp::uart::{Config, InterruptHandler, Uart};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    UART0_IRQ => InterruptHandler<UART0>;
    DMA_IRQ_0 => dma::InterruptHandler<DMA_CH0>, dma::InterruptHandler<DMA_CH1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut uart = Uart::new(
        p.UART0, p.PIN_0, p.PIN_1, Irqs, p.DMA_CH0, p.DMA_CH1, Config::default(),
    );

    uart.write(b"Hello World!\r\n").await.unwrap();

    let mut buf = [0u8; 1];
    loop {
        uart.read(&mut buf).await.unwrap();
        uart.write(&buf).await.unwrap();
    }
}
