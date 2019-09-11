#![no_std]
#![no_main]

mod gdt;
mod interrupts;
mod vga_driver;

use core::fmt::Write;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn kernel_main() {
    println!("Welcome to kernul OS !");
    println!();
    println!("For now it's pretty bad");

    println!("Initializing GDT (Global Descriptor Table)");
    gdt::init_gdt();
    println!("Initializing IDT (Interrupt Descriptor Table)");
    interrupts::idt::init_idt();
    panic!("Test panic");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
