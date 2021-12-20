use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};
use crate::sys::vga::WRITER;
use core::fmt::Write;
use crate::sys::vga::color::*;
pub extern "x86-interrupt" fn handle(frame: InterruptStackFrame, error_code: PageFaultErrorCode) {
    let mut console = WRITER.lock();
    console.color = ColorType::Custom(crate::color!(ColorCode::Blue, ColorCode::Red));
    writeln!(console, "EXCEPTION: PAGE FAULT").unwrap();
    writeln!(console, "Accessed Address: {:?}", frame.instruction_pointer).unwrap();
    writeln!(console, "Error Code: {:?}", error_code).unwrap();
    writeln!(console, "Stack: {:#?}", frame).unwrap();

    loop {}
}