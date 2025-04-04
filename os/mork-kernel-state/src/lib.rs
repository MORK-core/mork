#![no_std]
extern crate alloc;

use alloc::boxed::Box;
use lazy_static::lazy_static;
use spin::mutex::Mutex;
use mork_common::mork_kernel_log;
use mork_hal::context::{HALContext, HALContextTrait};
use mork_task::schedule::Scheduler;
use mork_task::task::TaskContext;
use mork_mm::page_table::PageTable;

lazy_static! {
    pub static ref KERNEL_ACCESS_DATA: Mutex<KernelSafeAccessData> = Mutex::new(KernelSafeAccessData::new());
}

pub struct KernelSafeAccessData {
    pub kernel_page_table: PageTable,
    pub scheduler: Scheduler,
    pub current_task: Option<Box<TaskContext>>,
    pub idle_task: Option<Box<TaskContext>>,
}

impl KernelSafeAccessData {
    fn new() -> Self {
        Self {
            kernel_page_table: PageTable::new(),
            scheduler: Scheduler::new(),
            current_task: None,
            idle_task: None,
        }
    }

    pub fn schedule(&mut self) -> *const HALContext {
        let current =
            if let Some(task) = self.scheduler.dequeue() {
                task.vspace.as_ref().unwrap().get().page_table_impl.active();
                task
            } else {
                mork_kernel_log!(info, "schedule idle thread");
                self.idle_task.take().unwrap()
            };
        let hal_context_pointer = current.as_ref().hal_context.get_pointer();
        self.current_task = Some(current);
        hal_context_pointer
    }
}