#![no_std]
#![no_main]

mod boot;
mod gpio;
mod panic;
mod uart;

fn kernel_main() -> ! {
    uart::init();
    uart::write_string("Hello world");

    loop {
        let c = uart::read_char();
        uart::write_char(c);
    }
}
