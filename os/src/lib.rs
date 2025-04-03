#![no_std]
#![no_main]

extern crate alloc;

use mork_common::mork_kernel_log;
use mork_kernel_state::KERNEL_ACCESS_DATA;
use mork_task::task_state::ThreadStateEnum;

mod auto_gen;
mod root_task;
mod lang_items;
mod trap;

#[unsafe(no_mangle)]
pub extern "C" fn rust_main(dtb_addr: usize) -> ! {
    mork_hal::init(dtb_addr).unwrap_or_else(|err| mork_kernel_log!(error, "{:?}", err));
    let mut kernel_access_data = KERNEL_ACCESS_DATA.lock();
    mork_mm::init(&mut kernel_access_data.kernel_page_table).unwrap_or_else(|err| mork_kernel_log!(error, "{:?}", err));
    trap::init();
    match root_task::init(&mut kernel_access_data) {
        Ok(mut root_task) => {
            root_task.state = ThreadStateEnum::ThreadStateRestart;
            kernel_access_data.scheduler.enqueue(root_task);
        }
        Err(err) => {
            mork_kernel_log!(error, "{:?}", err);
            mork_hal::shutdown(true)
        }
    }
    match root_task::init_idle_thread() {
        Ok(idle_task) => {
            kernel_access_data.idle_task = Some(idle_task);
        }
        Err(err) => {
            mork_kernel_log!(error, "{:?}", err);
            mork_hal::shutdown(true)
        }
    }
    let hal_context_pointer = kernel_access_data.schedule();
    drop(kernel_access_data);
    // mork_hal::timer::set_next_trigger();
    mork_hal::return_user(hal_context_pointer);
    mork_hal::shutdown(false)
}