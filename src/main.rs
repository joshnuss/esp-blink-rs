use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use log::info;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // enable logging
    esp_idf_svc::log::EspLogger::initialize_default();

    // get peripherals
    let peripherals = Peripherals::take()?;

    // built-in LED is GPIO8
    let mut led = PinDriver::output(peripherals.pins.gpio8)?;

    // alternate between high and low, with delay of 1s
    loop {
        info!("high");
        led.set_high()?;
        FreeRtos::delay_ms(1000);

        info!("low");
        led.set_low()?;
        FreeRtos::delay_ms(1000);
    }
}
