//! testosterone module provides testing utils

#[cfg(test)]
use crate::qemuno::{QemuExitCode, exit_qemu};

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

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    // new
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

#[test_case]
fn base_test() {
    assert_eq!(1, 1);
}
