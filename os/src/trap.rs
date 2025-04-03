use riscv::register::{scause, stval};
use mork_common::syscall::message_info::MessageInfo;
use mork_common::syscall::Syscall;
use mork_hal::context::HALContextTrait;
use mork_kernel_state::KERNEL_ACCESS_DATA;

pub fn init() {
    mork_hal::trap_init();
}

#[unsafe(no_mangle)]
pub fn handle_syscall(cptr: usize, msg_info: usize, syscall: isize) {
    let mut kernel_state = KERNEL_ACCESS_DATA.lock();
    mork_syscall::handle_syscall(
        &mut kernel_state,
        cptr,
        MessageInfo::from_word(msg_info),
        Syscall::from(syscall).unwrap()
    );

    let hal_context_pointer = kernel_state.schedule();
    drop(kernel_state);
    mork_hal::return_user(hal_context_pointer);
}

#[unsafe(no_mangle)]
pub fn handle_exception() {
    let mut kernel_state = KERNEL_ACCESS_DATA.lock();
    let current = kernel_state.current_task.take().unwrap();
    let scause = scause::read();
    let stval = stval::read();
    panic!("scause={:?}, stval={:#x}, bad instruction: {:#x}",
        scause.cause(), stval, current.hal_context.get_fault_ip());
}
