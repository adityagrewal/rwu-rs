#![no_std]
#![no_main]
 
use panic_rtt_target as _;
use esp_hal::{
    delay::Delay,
    gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull},
    main,
};

esp_bootloader_esp_idf::esp_app_desc!();
 
#[main]
fn main() -> ! {
    rtt_target::rtt_init_defmt!();
    
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut led = Output::new(peripherals.GPIO7, Level::Low, OutputConfig::default());
    let button = Input::new(peripherals.GPIO9, InputConfig::default().with_pull(Pull::Up));
    let delay = Delay::new();
 
    loop {
        if button.is_low() {
            led.set_high();
        } else {
            led.set_low();
            }
        delay.delay_millis(20);
    }
}
