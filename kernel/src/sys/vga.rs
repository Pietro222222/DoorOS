pub mod color;
use color::ColorCode;
use spin::Mutex;
use core::fmt::Write;
pub const BUFFER_WIDTH: usize = 80;
pub const BUFFER_HEIGHT: usize = 25;

pub struct ScreenChar {
    pub chr: u8,
    pub color: ColorCode,
}
pub struct Buffer {
    pub chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}
pub struct VGAWriter {
    pub buffer: &'static mut Buffer,
    pub color: ColorCode,
    pub row: usize,
    pub col: usize,
}

impl VGAWriter {
    pub fn write(&mut self, byte: u8) {
        if self.col >= BUFFER_WIDTH || byte == b'\n' {
            self.col = 0;
            self.row += 1;
        }
        if self.row >= BUFFER_HEIGHT {
            self.row = 0;
        }
        if byte != b'\n' { 
            let col = if self.col == 0 { 0 } else { self.col - 1 };
            self.buffer.chars[self.row][col] = ScreenChar {
                chr: byte,
                color: self.color,
            };
        }
        self.col += 1;
    }
}
impl Write for VGAWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            self.write(byte)
        }
        Ok(())
    }
}
lazy_static::lazy_static! {
    pub static ref WRITER: Mutex<VGAWriter> = Mutex::new(VGAWriter {
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        color: ColorCode::Green,
        row: 0,
        col: 0,
    });
}
