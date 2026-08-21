#![no_std]
#![no_main]

use panic_rtt_target as _;
use esp_hal::{
    analog::adc::{Adc, AdcConfig, Attenuation},
    delay::Delay,
    main,
};

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    rtt_target::rtt_init_defmt!();
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut adc_config = AdcConfig::new();
    let mut pin = adc_config.enable_pin(peripherals.GPIO2, Attenuation::_11dB);
    let mut adc1 = Adc::new(peripherals.ADC1, adc_config);
    let delay = Delay::new();

    loop {
        let value: u16 = nb::block!(adc1.read_oneshot(&mut pin)).unwrap();
        defmt::info!("adc: {}", value);
        delay.delay_millis(200);
    }
}
