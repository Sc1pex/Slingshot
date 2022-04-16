#![no_std]
#![no_main]

mod boot;
mod gpio;
mod panic;

fn kernel_main() -> ! {
    gpio::setup_pin(23, gpio::Mode::Output, gpio::Pull::None);
    gpio::set_output(23, gpio::Value::High);

    loop {}
}
