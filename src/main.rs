#![no_std]
#![no_main]
#![feature(asm)]
#![feature(global_asm)]
#![feature(format_args_nl)]
#![feature(panic_info_message)]

mod boot;
mod gpio;
mod panic;
mod uart;

fn kernel_main() -> ! {
    uart::init();
    uart::write_str("Hello, World");

    gpio::set_mode(gpio::Mode::Output, 23);
    gpio::set_value(gpio::Value::High, 23);

    loop {}
}
