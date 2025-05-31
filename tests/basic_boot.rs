#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osel::testosterone::test_runner)]
#![reexport_test_harness_main = "test_main"]

/// Integrational tests

use core::panic::PanicInfo;
#[cfg(not(test))]
use osel::testosterone::test_main;
use osel::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    osel::testosterone::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
