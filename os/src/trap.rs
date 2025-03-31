use mork_common::syscall::message_info::MessageInfo;
use mork_common::syscall::Syscall;
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