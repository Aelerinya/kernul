#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn kernel_main() {
    let buffer = 0xb8000 as *mut u8;
    unsafe {
        *buffer = 'z' as u8;
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
