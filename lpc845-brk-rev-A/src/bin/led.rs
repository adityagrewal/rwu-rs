#![no_std]
#![no_main]
use panic_halt as _;
use cortex_m_rt::entry;
use lpc8xx_hal::{cortex_m::peripheral::Peripherals as CorePeripherals, prelude::*, delay::Delay, gpio::Level, Peripherals};

#[entry]
fn main() -> ! {
    let cp = CorePeripherals::take().unwrap();
    let p = Peripherals::take().unwrap();

    let mut syscon = p.SYSCON.split();
    let gpio = p.GPIO.enable(&mut syscon.handle);
    let pins = p.pins;

    // active low: High is LED off
    let mut led = pins.pio1_2.into_output_pin(gpio.tokens.pio1_2, Level::High);
    let mut delay = Delay::new(cp.SYST);

    loop {
        led.toggle();
        delay.delay_ms(500u32);
    }
}
