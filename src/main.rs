#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osel::testosterone::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[macro_use]
mod cereal;
mod avganec;
mod qemuno;
mod testosterone;

#[cfg(not(test))]
pub use crate::testosterone::test_main;
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("OSel woke up!");
    osel::init();
    #[cfg(test)]
    test_main();
    println!("OSel ready to go!");
    loop {}
}

#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    testosterone::test_panic_handler(info);
}
