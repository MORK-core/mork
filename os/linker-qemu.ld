OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDRESS = 0xffffffff80200000;
OFFSET = 0xffffffff00000000;
SECTIONS
{
    . = BASE_ADDRESS;

    .text : AT(ADDR(.text) - OFFSET) {
        stext = .;
        *(.text.entry)
        *(.text .text.*)
        etext = .;
    }
    . = ALIGN(4K);
    .root_task_data : AT(ADDR(.root_task_data) - OFFSET) {
        root_task_data_start = .;
        KEEP(*(.root_task_data))
        root_task_data_end = .;
    }

    . = ALIGN(4K);
    .rodata : AT(ADDR(.rodata) - OFFSET) {
        srodata = .;
        *(.rodata .rodata.*)
        erodata = .;
    }

    . = ALIGN(4K);
    .data : AT(ADDR(.data) - OFFSET) {
        sdata = .;
        *(.data .data.*)
        edata = .;
    }

    . = ALIGN(4K);
    sbss_with_stack = .;
    .bss : AT(ADDR(.bss) - OFFSET) {
	    *(.bss.boot.stack)
        sbss = .;
        *(.sbss .bss .bss.*)
        ebss = .;
    }
    . = ALIGN(4K);

    kernel_end = .;
    PROVIDE(end = .);
}