use x86_64::structures::idt::InterruptDescriptorTable;

/// initializes Interrupt Descriptor Table
pub fn init_idt() {
    let mut idt = InterruptDescriptorTable::new();
}
