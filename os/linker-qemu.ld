OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDRESS = 0x80200000;
SECTIONS
{
    . = BASE_ADDRESS;

    .text : AT(ADDR(.text)) {
        stext = .;
        *(.text.entry)
        *(.text .text.*)
        etext = .;
    }
    . = ALIGN(4K);
    .root_task_data : AT(ADDR(.root_task_data)) {
        root_task_data_start = .;
        KEEP(*(.root_task_data))
        root_task_data_end = .;
    }

    . = ALIGN(4K);
    .rodata : AT(ADDR(.rodata)) {
        srodata = .;
        *(.rodata .rodata.*)
        erodata = .;
    }

    . = ALIGN(4K);
    .data : AT(ADDR(.data)) {
        sdata = .;
        *(.data .data.*)
        edata = .;
    }


    . = ALIGN(4K);
    .kernel : AT(ADDR(.kernel)) {
        *(.kernel.idle_thread)
    }

    . = ALIGN(4K);
    sbss_with_stack = .;
    .bss : AT(ADDR(.bss)) {
        *(.bss.root_pagetable)
        *(.bss.l2_pagetable)
	    *(.bss.boot.stack)
        sbss = .;
        *(.sbss .bss .bss.*)
        ebss = .;
    }
    . = ALIGN(4K);

    kernel_end = .;
    PROVIDE(end = .);
}