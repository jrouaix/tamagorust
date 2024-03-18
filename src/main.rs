#![no_std]
#![no_main]

// use embassy_executor::Spawner;
// use embassy_time::{Duration, Timer};
// use esp_idf_hal::{gpio::PinDriver, peripherals::Peripherals, sys::EspError};
// use esp_idf_svc::{ timer::embassy_time_driver }

use core::mem::MaybeUninit;

use esp_hal::{clock::ClockControl, entry, peripherals::Peripherals, system::SystemExt, Delay};
use esp_println::println;

#[global_allocator]
static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

fn init_heap() {
  const HEAP_SIZE: usize = 32 * 1024;
  static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

  unsafe {
    ALLOCATOR.init(HEAP.as_mut_ptr() as *mut u8, HEAP_SIZE);
  }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}

#[entry]
fn main() -> ! {
  init_heap();
  let peripherals = Peripherals::take();
  let system = peripherals.SYSTEM.split();

  let clocks = ClockControl::max(system.clock_control).freeze();
  let mut delay = Delay::new(&clocks);

  // setup logger
  // To change the log_level change the env section in .cargo/config.toml
  // or remove it and set ESP_LOGLEVEL manually before running cargo run
  // this requires a clean rebuild because of https://github.com/rust-lang/cargo/issues/10358
  esp_println::logger::init_logger_from_env();
  log::info!("Logger is setup");
  println!("Hello world!");
  // let timer = TimerGroup::new(peripherals.TIMG1, &clocks).timer0;
  // let _init = initialize(EspWifiInitFor::Wifi, timer, Rng::new(peripherals.RNG), system.radio_clock_control, &clocks).unwrap();
  loop {
    //   println!("Loop...");
    delay.delay_micros(500u32);
  }
}

// #[no_mangle]
// fn main() {
//   // It is necessary to call this function once. Otherwise some patches to the runtime
//   // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
//   // esp_idf_svc::sys::link_patches();

//   // Bind the log crate to the ESP Logging facilities
//   // esp_idf_svc::log::EspLogger::initialize_default();

//   // let mut executor = ::embassy_executor::Executor::new();
//   //           let executor = unsafe { __make_static(&mut executor) };
//   //           executor.run(|spawner| {
//   //               spawner.must_spawn(__embassy_main(spawner));
//   //           })

//   // Ok(())
// }

// #[no_mangle]
// fn main() -> Result<(), EspError> {

//   log::info!("Hello, world!");

//   let peripherals = Peripherals::take()?;
//   let mut led = PinDriver::output(peripherals.pins.gpio10)?;

//   for _ in 0..100 {
//     log::info!("LED ON");
//     led.set_high()?;
//     Timer::after(Duration::from_millis(1_000)).await;

//     log::info!("LED OFF");
//     led.set_low()?;
//     // sleep(Duration::from_secs(1));
//   }

//   Ok(())
// }
