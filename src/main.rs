#![no_std]
#![no_main]

mod avganec;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("OSel woke up and ready to go!");
    loop {}
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
