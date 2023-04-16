#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic( info: &PanicInfo) -> ! {
  loop {}
}

#[ardunio_hal::entry]
fn main() -> ! {
  loop {}
}