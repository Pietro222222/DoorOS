#![no_std]
#![no_main]

use kernel::kernel as zkernel;
use kernel::set_panic_hook;
use kernel::sys::mem;
use kernel::sys::idt;
use bootloader::BootInfo;
set_panic_hook!();
#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    mem::init(&boot_info);
    idt::init();
    zkernel::init();
    loop {}
}
