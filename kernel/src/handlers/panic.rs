#[macro_export]
macro_rules! set_panic_hook {
    () => {
        use core::panic::PanicInfo;

        /// This function is called on panic.
        #[panic_handler]
        fn panic(_info: &PanicInfo) -> ! {
            let mut writer = kernel::sys::vga::WRITER.lock();
            writer.color = kernel::sys::vga::color::ColorCode::Red;
            use core::fmt::Write;
            writeln!(writer, "Kernel panic: {}", _info).unwrap();
            
            loop {}
        }
    };
}
