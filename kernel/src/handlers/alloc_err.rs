#[macro_export]
macro_rules! set_alloc_error_hook {
    () => {
        #[alloc_error_handler]
        fn on_err(e: alloc::alloc::Layout) -> ! {
            panic!("Allocation error: {:?}", e);
        }
    }
}