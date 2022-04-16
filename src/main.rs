#![no_std]
#![no_main]

mod boot;
mod gpio;
mod panic;

fn kernel_main() -> ! {
    loop {}
}
