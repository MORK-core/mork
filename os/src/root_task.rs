use alloc::alloc::alloc;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use core::alloc::Layout;
use mork_common::utils::alignas::{align_down, align_up};
use elf::abi::{PF_R, PF_W, PF_X, PT_LOAD};
use elf::ElfBytes;
use elf::endian::AnyEndian;
use mork_common::mork_kernel_log;
use mork_common::types::{ResultWithErr, SyncUnsafeCell};
use mork_hal::context::HALContextTrait;
use mork_hal::KERNEL_OFFSET;
use mork_hal::mm::PageTableImpl;
use mork_kernel_state::KernelSafeAccessData;
use mork_mm::page_table::MutPageTableWrapper;
use mork_task::task::TaskContext;

pub fn init(kernel_access_data: &mut KernelSafeAccessData) -> Result<TaskContext ,String> {
    let mut root_task = TaskContext::new_user_thread();
    let (start, end) = mork_hal::get_root_task_region()?;
    mork_kernel_log!(info, "root task image region: {:#x} --> {:#x}", start, end);
    let elf_data = unsafe {
        core::slice::from_raw_parts(start as *const u8, end - start)
    };
    let elf = ElfBytes::<AnyEndian>::minimal_parse(elf_data).unwrap();
    let mut page_table = kernel_access_data.kernel_page_table;
    init_vspace(&mut page_table, &elf, start - KERNEL_OFFSET)?;
    root_task.vspace = Some(Arc::new(SyncUnsafeCell::new(page_table)));
    mork_kernel_log!(info, "root task entry: {:#x}", elf.ehdr.e_entry);
    root_task.hal_context.set_next_ip(elf.ehdr.e_entry as usize);
    Ok(root_task)
}

fn init_vspace(vspace: &mut PageTableImpl, elf: &ElfBytes<AnyEndian>, p_base: usize) -> ResultWithErr<String> {
    let mut root_vspace_wrapper = MutPageTableWrapper::new(vspace);
    let segments = elf.segments().expect("could not parse ELF segments");
    const PAGE_ALIGN: usize = 4096;
    for segment in segments {
        if segment.p_type != PT_LOAD {
            continue;
        }

        mork_kernel_log!(info,
            "Segment: offset=0x{:#x}, vaddr={:#x}, file size={:#x}, mem size: {:#x}, flags={:x}",
            segment.p_offset, segment.p_vaddr, segment.p_filesz, segment.p_memsz, segment.p_flags
        );

        let mut vaddr = align_down(segment.p_vaddr as usize, PAGE_ALIGN);
        let need_alloc = segment.p_filesz == segment.p_memsz;
        let end = if need_alloc {
            align_up(segment.p_vaddr as usize + segment.p_filesz as usize, PAGE_ALIGN)
        } else {
            align_up(segment.p_vaddr as usize + segment.p_memsz as usize, PAGE_ALIGN)
        };
        let mut paddr =  if need_alloc {
            p_base + segment.p_offset as usize
        } else {
            let layout = Layout::from_size_align(
                align_up(segment.p_memsz as usize, PAGE_ALIGN), PAGE_ALIGN)
                .unwrap();
            unsafe { alloc(layout) as usize - KERNEL_OFFSET }
        };
        while vaddr < end {
            root_vspace_wrapper
                .map_normal_frame(
                    vaddr,
                    paddr,
                    segment.p_flags & PF_X != 0,
                    segment.p_flags & PF_W != 0,
                    segment.p_flags & PF_R != 0)?;
            vaddr += 4096;
            paddr += 4096;
        }
    }
    Ok(())
}

pub fn init_idle_thread() -> Result<Box<TaskContext>, String> {
    const IDLE_THREAD_STACK_SIZE: usize = 4096;
    const STACK_ALIGN: usize = 4096;
    let layout = Layout::from_size_align(IDLE_THREAD_STACK_SIZE, STACK_ALIGN).unwrap();
    let sp = unsafe { alloc(layout) };
    if sp.is_null() {
        return Err("could not allocate stack memory".to_string());
    }
    let idle_task = Box::new(TaskContext::new_idle_thread(
        sp.addr() + IDLE_THREAD_STACK_SIZE));
    Ok(idle_task)
}