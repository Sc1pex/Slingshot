#![no_std]
#![no_main]
#![feature(asm)]
#![feature(global_asm)]
#![feature(format_args_nl)]
#![feature(panic_info_message)]

mod boot;
mod driver;
mod gpio;
mod panic;
mod uart;

unsafe fn kernel_init() -> ! {
    driver::driver_manager().init();

    kernel_main()
}

fn kernel_main() -> ! {
    uart::init();
    uart::write_str("Hello, World\n");

    // gpio::set_mode(gpio::Mode::Output, 23);
    // gpio::set_value(gpio::Value::High, 23);

    loop {
        if let Some(x) = uart::read_char(false) {
            uart::write_char(x);
        }
    }
}
