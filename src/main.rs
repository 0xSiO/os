#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log::error!("{}", info);
    os::halt();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::testing::test_panic_handler(info)
}

// Entry point for starting the OS, or running main tests
fn main(boot_info: &'static BootInfo) -> ! {
    os::initialize(boot_info);

    let mut executor = os::task::BasicExecutor::new();
    executor.spawn(os::task::print_keypresses());
    executor.run();

    #[cfg(test)]
    test_main();

    os::halt();
}

entry_point!(main);
