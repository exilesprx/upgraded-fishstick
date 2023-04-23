#![no_std]
#![no_main]

use core::panic::PanicInfo;

use arduino_hal::Peripherals;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}

#[arduino_hal::entry]
fn main() -> ! {
  let peripherals: Peripherals = Peripherals::take().unwrap();
  let pins = arduino_hal::pins!(peripherals);
  let mut led = pins.d13.into_output();

  loop {
    led.toggle();
    arduino_hal::delay_ms(1000);
  }
}