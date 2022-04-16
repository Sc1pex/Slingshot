__load_addr = 0x80000;

SECTIONS
{
    . = __load_addr;
    .text : {
        KEEP(*(.text.boot))
        *(.text*)
    }
    .rodata : {
        *(.rodata*)
    }
    .data : {
        *(.data*)
    }
    .bss (NOLOAD) : {
        . = ALIGN(16);
        __bss_start = .;
        *(.bss*)
        *(COMMON)
        __bss_end = .;
    }
}