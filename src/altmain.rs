#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    halt()
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Nothing exists yet. Write to VGA text memory directly to prove we ran.
    unsafe {
        let vga = 0xb8000 as *mut u8;
        let msg = b"Marquays: alive";
        let mut i = 0;
        while i < msg.len() {
            vga.add(i * 2).write_volatile(msg[i]);
            vga.add(i * 2 + 1).write_volatile(0x0f); // white on black
            i += 1;
        }
    }
    halt()
}

fn halt() -> ! {
    loop {
        unsafe {
            core::arch::asm!("cli", "hlt", options(nomem, nostack, preserves_flags));
        }
    }
}