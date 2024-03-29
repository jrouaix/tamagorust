use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::peripherals::Peripherals;
use std::{thread::sleep, time::Duration};

// use embassy_executor::raw::Executor;

fn main() -> anyhow::Result<()> {
  // It is necessary to call this function once. Otherwise some patches to the runtime
  // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
  esp_idf_svc::sys::link_patches();

  // Bind the log crate to the ESP Logging facilities
  esp_idf_svc::log::EspLogger::initialize_default();

  log::info!("Hello, world!");

  let peripherals = Peripherals::take()?;
  let mut led = PinDriver::output(peripherals.pins.gpio10)?;

  // const plop: () = ();
  // let executor = Executor::new(&mut plop);

  loop {
    log::info!("LED ON");
    led.set_high()?;
    sleep(Duration::from_secs(1));

    log::info!("LED OFF");
    led.set_low()?;
    sleep(Duration::from_secs(1));
  }

  //   use esp_idf_svc::{
  //   eventloop::EspSystemEventLoop,
  //   nvs::EspDefaultNvsPartition,
  //   wifi::{AuthMethod, ClientConfiguration, Configuration, EspWifi},
  // };
  // let sysloop = EspSystemEventLoop::take()?;

  // loop {
  //   log::info!("LOOP");
  //   sleep(Duration::from_secs(1));
  // }

  // let nvs = EspDefaultNvsPartition::take()?;

  // let mut wifi = EspWifi::new(peripherals.modem, sysloop, Some(nvs))?;

  // wifi.set_configuration(&Configuration::Client(ClientConfiguration {
  //   ssid: "NOPE".parse().unwrap(),
  //   password: "NOPE".parse().unwrap(),
  //   auth_method: AuthMethod::None,
  //   ..Default::default()
  // }))?;

  // // Start Wifi
  // wifi.start()?;

  // // Connect Wifi
  // wifi.connect()?;

  // // Confirm Wifi Connection
  // while !wifi.is_connected().unwrap() {
  //   // Get and print connection configuration
  //   let config = wifi.get_configuration().unwrap();
  //   println!("Waiting for station {:?}", config);
  // }

  // println!("Connected");
  // Ok(())
}
