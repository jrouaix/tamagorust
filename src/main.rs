use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::peripherals::Peripherals;
use std::{thread::sleep, time::Duration};

// Declare async tasks
#[embassy_executor::task]
async fn blink(pin: AnyPin) {
  let mut led = Output::new(pin, Level::Low, OutputDrive::Standard);

  loop {
    // Timekeeping is globally available, no need to mess with hardware timers.
    led.set_high();
    Timer::after_millis(150).await;
    led.set_low();
    Timer::after_millis(150).await;
  }
}

// Main is itself an async task as well.
#[embassy_executor::main]
async fn main(spawner: Spawner) {
  // It is necessary to call this function once. Otherwise some patches to the runtime
  // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
  esp_idf_svc::sys::link_patches();

  // Bind the log crate to the ESP Logging facilities
  esp_idf_svc::log::EspLogger::initialize_default();

  let peripherals = Peripherals::take()?;
  let mut led = PinDriver::output(peripherals.pins.gpio10)?;

  // Spawned tasks run in the background, concurrently.
  spawner.spawn(blink(led.degrade())).unwrap();

  let mut button = Input::new(p.P0_11, Pull::Up);
  loop {
    // Asynchronously wait for GPIO events, allowing other tasks
    // to run, or the core to sleep.
    button.wait_for_low().await;
    info!("Button pressed!");
    button.wait_for_high().await;
    info!("Button released!");
  }
}
