#![no_std] // Don't link the std lib
#![no_main] // No default main entry point
use core::panic::PanicInfo;

// This gets called when a panic occurs
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle] // Don't mangle the function name
pub extern "C" fn _start() -> !{
    // This function acts as the program entry point
    loop{}
}