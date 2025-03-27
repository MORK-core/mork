use alloc::string::String;
use alloc::sync::Arc;
use mork_common::utils::alignas::{align_down, align_up};
use elf::abi::{PF_R, PF_W, PF_X, PT_LOAD};
use elf::ElfBytes;
use elf::endian::AnyEndian;
use log::info;
use mork_common::types::{ResultWithErr, SyncUnsafeCell};
use mork_hal::mm::PageTableImpl;
use mork_mm::page_table::MutPageTableWrapper;
use mork_task::task::TaskContext;

pub fn init() -> Result<TaskContext ,String> {
    let mut root_task = TaskContext::new();
    let (start, end) = mork_hal::get_root_task_region()?;
    let elf_data = unsafe {
        core::slice::from_raw_parts(start as *const u8, end - start)
    };
    let elf = ElfBytes::<AnyEndian>::minimal_parse(elf_data).unwrap();
    let mut page_table = PageTableImpl::new();
    init_vspace(&mut page_table, &elf)?;
    root_task.vspace = Some(Arc::new(SyncUnsafeCell::new(page_table)));
    Ok(root_task)
}

fn init_vspace(vspace: &mut PageTableImpl, elf: &ElfBytes<AnyEndian>) -> ResultWithErr<String> {
    let mut root_vspace_wrapper = MutPageTableWrapper::new(vspace);
    let segments = elf.segments().expect("could not parse ELF segments");
    for segment in segments {
        if segment.p_type != PT_LOAD {
            continue;
        }
        info!(
            "Segment: offset=0x{:x}, vaddr=0x{:x}, size={}, flags={:x}",
            segment.p_offset, segment.p_vaddr, segment.p_filesz, segment.p_flags
        );
        let mut vaddr = align_down(segment.p_vaddr as usize, 4096);
        let end = align_up(segment.p_vaddr as usize + segment.p_filesz as usize, 4096);
        let mut paddr =  vaddr + segment.p_offset as usize;
        while vaddr < end {
            root_vspace_wrapper
                .map_normal_frame(
                    vaddr,
                    paddr,
                    segment.p_flags & PF_X != 0,
                    segment.p_flags & PF_R != 0,
                    segment.p_flags & PF_W != 0)?;
            vaddr += 4096;
            paddr += 4096;
        }
    }
    Ok(())
}