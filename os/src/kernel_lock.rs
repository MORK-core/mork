use mork_hal::mm::PageTableImpl;
use spin::Mutex;
use lazy_static::lazy_static;
use mork_task::schedule::Scheduler;

lazy_static! {
    pub static ref KERNEL_ACCESS_DATA: Mutex<KernelSafeAccessData> = Mutex::new(KernelSafeAccessData::new());
}

pub struct KernelSafeAccessData {
    pub kernel_page_table: PageTableImpl,
    pub scheduler: Scheduler,
}

impl KernelSafeAccessData {
    fn new() -> Self {
        Self {
            kernel_page_table: PageTableImpl::new(),
            scheduler: Scheduler::new(),
        }
    }
}