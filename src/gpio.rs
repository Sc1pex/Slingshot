#![allow(dead_code)]

use core::ptr::{read_volatile, write_volatile};

pub const PERIPHERAL_BASE: u32 = 0xfe000000;
const GPIO_BASE: u32 = PERIPHERAL_BASE + 0x200000;
const MAX_PIN: u32 = 53;

pub enum Mode {
    Input,
    Output,
    Alt0,
    Alt1,
    Alt2,
    Alt3,
    Alt4,
    Alt5,
}
pub fn set_mode(pin: u32, mode: Mode) {
    if pin > MAX_PIN {
        // TODO: panic
        return;
    }

    let register = pin / 10 * 4 + GPIO_BASE + 0x00;
    let offset = pin % 10 * 3;
    unsafe {
        match mode {
            Mode::Input => write_bits(register, offset, 0b000, 3),
            Mode::Output => write_bits(register, offset, 0b001, 3),
            Mode::Alt0 => write_bits(register, offset, 0b100, 3),
            Mode::Alt1 => write_bits(register, offset, 0b101, 3),
            Mode::Alt2 => write_bits(register, offset, 0b110, 3),
            Mode::Alt3 => write_bits(register, offset, 0b111, 3),
            Mode::Alt4 => write_bits(register, offset, 0b011, 3),
            Mode::Alt5 => write_bits(register, offset, 0b010, 3),
        }
    }
}

pub enum Value {
    High,
    Low,
}
pub fn set_output(pin: u32, value: Value) {
    if pin > MAX_PIN {
        // TODO: panic
        return;
    }

    match value {
        Value::High => {
            let register = pin / 32 * 4 + GPIO_BASE + 0x1c;
            let offset = pin % 32;

            unsafe {
                write_bits(register, offset, 1, 1);
            }
        }
        Value::Low => {
            let register = pin / 32 * 4 + GPIO_BASE + 0x28;
            let offset = pin % 32;

            unsafe {
                write_bits(register, offset, 1, 1);
            }
        }
    }
}

pub enum Pull {
    Up,
    Down,
    None,
}
pub fn set_pull(pin: u32, pull: Pull) {
    if pin > MAX_PIN {
        // TODO: panic
        return;
    }

    let register = pin / 16 * 4 + GPIO_BASE + 0xe4;
    let offset = pin % 16 * 2;
    unsafe {
        match pull {
            Pull::None => write_bits(register, offset, 0b00, 2),
            Pull::Down => write_bits(register, offset, 0b10, 2),
            Pull::Up => write_bits(register, offset, 0b01, 2),
        }
    }
}

unsafe fn write_bits(register: u32, offset: u32, value: u32, size: u32) {
    let mask = !((1 << (size + 1) - 1) << offset);
    let mut current = read_volatile(register as *const u32);
    current &= mask;
    current |= value << offset;

    write_volatile(register as *mut u32, current);
}

pub fn setup_pin(pin: u32, mode: Mode, pull: Pull) {
    set_mode(pin, mode);
    set_pull(pin, pull);
}
