use crate::sys::vga::WRITER;
use core::fmt::Write;


lazy_static::lazy_static! {
 pub static ref CONSOLE: Console = Console::default();
}
pub struct Console;
impl Console {
    pub fn write(&self, text: &str) {
        write!(WRITER.lock(), "{}", text).unwrap();
    }
    #[inline]
    pub fn log(&self, text: &str) {
        writeln!(WRITER.lock(), "{}", text).unwrap();
    }
}
impl Default for Console {
    fn default() -> Self {
        Self {}
    }
}
