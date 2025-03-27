#![no_std]
#![no_main]

extern crate alloc;

use alloc::boxed::Box;
use log::error;
use crate::kernel_lock::KERNEL_ACCESS_DATA;

mod auto_gen;
mod root_task;
mod kernel_lock;

#[unsafe(no_mangle)]
pub extern "C" fn rust_main(dtb_addr: usize) -> ! {
    mork_hal::init(dtb_addr).unwrap_or_else(|err| error!("{:?}", err));
    let mut kernel_access_data = KERNEL_ACCESS_DATA.lock();
    mork_mm::init(&mut kernel_access_data.kernel_page_table).unwrap_or_else(|err| error!("{:?}", err));
    match root_task::init() {
        Ok(root_task) => {
            kernel_access_data.scheduler.enqueue(Box::new(root_task));
            let task = kernel_access_data.scheduler.dequeue().unwrap();
            task.vspace.as_ref().unwrap().get_mut().map_page_table(0, 0, 0);
        }
        Err(err) => {
            error!("{:?}", err);
            mork_hal::shutdown(true)
        }
    }
    mork_hal::shutdown(false)
}