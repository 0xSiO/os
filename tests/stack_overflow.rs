#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};
use conquer_once::spin::Lazy;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use os::{qemu, serial_print, serial_println};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::testing::test_panic_handler(info)
}

// Set up a custom interrupt descriptor table with a handler function that
// prints [ok] and exits qemu.
static TEST_IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    let mut table = InterruptDescriptorTable::new();
    unsafe {
        table
            .double_fault
            .set_handler_fn(test_double_fault_handler)
            .set_stack_index(os::gdt::DOUBLE_FAULT_IST_INDEX);
    }
    table
});

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: &mut InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[ok]");
    qemu::exit(qemu::ExitCode::Success);
    os::halt();
}

fn main(boot_info: &'static BootInfo) -> ! {
    serial_print!("stack_overflow... ");

    // Main initialization sequence, using test IDT
    TEST_IDT.load();
    os::gdt::initialize_global_descriptor_table();
    os::interrupts::initialize_interrupt_controller();
    os::memory::initialize_heap_allocator(boot_info);

    #[allow(unconditional_recursion)]
    fn stack_overflow() {
        stack_overflow();
    }
    stack_overflow();

    panic!("Continued running after stack overflow!");
}

entry_point!(main);
