core::arch::global_asm!(include_str!("boot.s"));

#[no_mangle]
pub unsafe fn __rust_init() -> ! {
    crate::kernel_main();
}
