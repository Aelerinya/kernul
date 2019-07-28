#![no_std]
#![no_main]

mod vga_driver;

use core::fmt::Write;
use core::panic::PanicInfo;
use vga_driver::VgaScreen;

#[no_mangle]
pub extern "C" fn kernel_main() {
    let mut screen = VgaScreen::new();

    writeln!(screen, "Welcome to kernul OS !").unwrap();
    writeln!(screen, "For now it's pretty bad").unwrap();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
