#![no_std]
#![no_main]

use core::panic::PanicInfo;

use os::{qemu, serial_print, serial_println};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    qemu::exit(qemu::ExitCode::Success);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_panic();
    serial_println!("[test did not panic]");
    qemu::exit(qemu::ExitCode::Failed);
    loop {}
}

fn should_panic() {
    serial_print!("should_panic... ");
    assert_eq!(0, 1);
}
