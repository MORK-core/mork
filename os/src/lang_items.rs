//! The panic handler

use core::panic::PanicInfo;
use mork_common::mork_kernel_log;
use mork_hal::shutdown;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        mork_kernel_log!(error,
            "[kernel] Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message()
        );
    } else {
        mork_kernel_log!(error, "[kernel] Panicked: {}", info.message());
    }
    shutdown(true)
}