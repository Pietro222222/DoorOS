use crate::api::console::CONSOLE as console;

const START_MESSAGE: &str = concat!("[SYSTEM] Welcome to DoorOS v", env!("CARGO_PKG_VERSION"));

pub fn init() {
    console.log(START_MESSAGE);
}
