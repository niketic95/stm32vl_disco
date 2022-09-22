#![no_std]
#![no_main]

use core::panic::PanicInfo;


#[panic_handler]
fn halt_handler(_info: &PanicInfo) -> !{
    loop{

    };
}