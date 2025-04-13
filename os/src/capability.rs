use alloc::boxed::Box;
use mork_capability::free_callback::register_handler;
use mork_syscall::DeallocHandler;

pub fn init() {
    register_handler(Box::new(DeallocHandler));
}