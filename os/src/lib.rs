#![no_std]
#![no_main]

use mork_hal::boot_init;

#[unsafe(no_mangle)]
pub extern "C" fn rust_main(dtb_addr: usize) -> ! {
    boot_init(dtb_addr);
}