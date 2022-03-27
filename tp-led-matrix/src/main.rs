#![no_std]
#![no_main]

use cortex_m_rt::entry;
use stm32l4 as _;

#[panic_handler]
fn panic_handler(_panic_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main() -> ! {
    panic!("The program stopped");
}