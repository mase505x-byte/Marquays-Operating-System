#![no_std]
#![no_main]

use crate::marq::kernel;
use crate::marq::kernel::core_local::{core_id, core_scheduler};
use crate::marq::kernel::interrupts; 
use crate::env::{self, StartInfo}; 
use crate::{console, drivers, executor, fs, logging, mm, syscalls};

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {
        unsafe { core::arch::asm!("hlt") };
    }
}
#[no_mangle]
pub extern "C" fn _start(boot_info: &'static StartInfo) -> ! {
    console::init_early();
    kernel_main(boot_info);

    halt()
}

fn halt() -> ! {
    loop {
        unsafe { 
            core::arch::asm!("cli"); 
            core::arch::asm!("hlt"); 
        }
    }
}
pub fn kernel_main(boot_info: &'static StartInfo) {
    logging::init();
    mm::init(&boot_info.memory_map);
    interrupts::init();

    marq::hal::init_target_hardware();

    drivers::init();
    fs::init();
    syscalls::init();

    marq::reconfig::init();
}