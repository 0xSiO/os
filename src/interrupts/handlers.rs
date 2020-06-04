use super::{InterruptIndex, PICS};
use crate::{keyboard, print};
use log::error;
use pc_keyboard::DecodedKey;
use x86_64::structures::idt::InterruptStackFrame;

pub(crate) extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    error!("EXCEPTION: breakpoint\n{:#?}", stack_frame);
}

pub(crate) extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: double fault\n{:#?}", stack_frame);
}

pub(crate) extern "x86-interrupt" fn timer_handler(_stack_frame: &mut InterruptStackFrame) {
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer as u8);
    }
}

pub(crate) extern "x86-interrupt" fn keyboard_handler(_stack_frame: &mut InterruptStackFrame) {
    if let Ok(Some(key)) = keyboard::decode_key() {
        match key {
            DecodedKey::Unicode(character) => print!("{}", character),
            DecodedKey::RawKey(key) => print!("{:?}", key),
        }
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard as u8);
    }
}
