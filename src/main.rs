#![no_std]
#![no_main]
#![feature(asm)]
#![feature(global_asm)]
#![feature(format_args_nl)]
#![feature(panic_info_message)]

mod boot;
mod gpio;
mod panic;

fn kernel_main() -> ! {
    gpio::set_mode(gpio::Mode::Output, 23);
    gpio::set_value(gpio::Value::High, 23);

    loop {}
}
