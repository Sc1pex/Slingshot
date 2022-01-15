.section ".text.boot"
.global _start

_start:
    mrs x1, mpidr_el1
    and x1, x1, #3
    cbz x1, init// If not on the main core, loop

loop:
    wfe
    b loop

init: 
    // Clear bss
    ldr x1, __bss_start
    ldr x2, __bss_end
clear_bss:
    cmp x1, x2
    b.eq boot
    stp xzr, xzr, [x0], #16

boot:
    // Move stack pointer
    ldr x1, __load_addr
    mov sp, x1
    b __rust_init
