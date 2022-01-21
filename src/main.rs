#![no_std]
#![no_main]
#![feature(asm)]
#![feature(global_asm)]
#![feature(format_args_nl)]
#![feature(panic_info_message)]

mod boot;
mod driver;
mod panic;
mod sync;
mod uart;

fn kernel_main() -> ! {
    driver::driver_manager().init();

    uart::init();
    uart::write_str("Hello, World\n");

    loop {
        if let Some(x) = uart::read_char(false) {
            uart::write_char(x);
        }
    }
}
