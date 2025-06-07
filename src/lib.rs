#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::testosterone::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(not(test))]
#[allow(unused_imports)]
use crate::testosterone::test_main;
#[cfg(test)]
use crate::testosterone::test_panic_handler;
#[cfg(test)]
use core::panic::PanicInfo;

pub mod avganec;
pub mod cereal;
pub mod qemuno;
pub mod testosterone;
pub mod interrupts;

pub fn init() {
    interrupts::init_idt();
}

/// Entry point for `cargo test`
#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
