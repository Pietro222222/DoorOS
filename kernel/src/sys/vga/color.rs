#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ColorCode {
    White = 0xF,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    LightGray = 0x7,
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xA,
    LightCyan = 0xB,
    LightRed = 0xC,
    LightMagenta = 0xD,
    Yellow = 0xE,
    Black = 0x0,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColorType {
    Custom(u8),
    Default(ColorCode),
}
#[macro_export]
macro_rules! color {
    ($fg:expr, $bg:expr) => {
        (($fg as u8) << 4 | $bg as u8) as u8
    };
}