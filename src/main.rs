// main.rs

#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {
        const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

        let hello = b"Hello, World!";
        for (i, &byte) in hello.iter().enumerate() {
            unsafe {
                *VGA_BUFFER.offset(i as isize * 2) = byte;
                *VGA_BUFFER.offset(i as isize * 2 + 1) = 0xb; // Light cyan color
            }
        }
    }
}