use std::{thread::sleep, time::Duration};
// use esp_idf_hal::peripherals::Peripherals;
// use esp_idf_svc::{
//   eventloop::EspSystemEventLoop,
//   nvs::EspDefaultNvsPartition,
//   wifi::{AuthMethod, ClientConfiguration, Configuration, EspWifi},
// };

fn main() -> anyhow::Result<()> {
  // It is necessary to call this function once. Otherwise some patches to the runtime
  // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
  esp_idf_svc::sys::link_patches();

  // Bind the log crate to the ESP Logging facilities
  esp_idf_svc::log::EspLogger::initialize_default();

  log::info!("Hello, world!");

  loop {
    log::info!("LOOP");
    sleep(Duration::from_secs(1));
  }

  // // Configure Wifi
  // let peripherals = Peripherals::take().unwrap();
  // let sysloop = EspSystemEventLoop::take()?;
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
