mod page_fault;

use x86_64::structures::idt::InterruptDescriptorTable;
lazy_static::lazy_static!(
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.page_fault.set_handler_fn(page_fault::handle);            
        idt
    };
);
pub fn init() {
    IDT.load();
}
