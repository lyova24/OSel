//! testosterone module provides testing utils

use crate::qemuno::QemuExitCode;
use crate::qemuno::exit_qemu;
use crate::serial_print;
use crate::serial_println;
use core::panic::PanicInfo;

#[allow(dead_code)]
pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

#[allow(dead_code)]
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run(); // new
    }
    exit_qemu(QemuExitCode::Success);
}

#[cfg(not(test))]
pub fn test_main() {
    // dummy function to void error in IDE. Not affecting build or execution, just IDE highlights
}

#[allow(dead_code)]
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}
