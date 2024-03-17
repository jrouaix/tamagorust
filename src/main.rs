#![no_std]
#![no_main]

use esp_idf_hal::{gpio::PinDriver, peripherals::Peripherals, sys::EspError};

#[no_mangle]
fn main() -> Result<(), EspError> {
  // It is necessary to call this function once. Otherwise some patches to the runtime
  // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
  esp_idf_svc::sys::link_patches();

  // Bind the log crate to the ESP Logging facilities
  esp_idf_svc::log::EspLogger::initialize_default();

  log::info!("Hello, world!");

  let peripherals = Peripherals::take()?;
  let mut led = PinDriver::output(peripherals.pins.gpio10)?;

  for _ in 0..100 {
    log::info!("LED ON");
    led.set_high()?;
    // sleep(Duration::from_secs(1));

    log::info!("LED OFF");
    led.set_low()?;
    // sleep(Duration::from_secs(1));
  }

  Ok(())
}
