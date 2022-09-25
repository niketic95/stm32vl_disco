#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;
use cortex_m_rt::entry;

#[panic_handler]
fn halt_handler(_info: &PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main() -> ! {
    loop {
        unsafe {
            asm!("nop");
        }
    }
}
