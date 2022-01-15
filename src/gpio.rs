#![allow(dead_code)]

use core::ptr::{read_volatile, write_volatile};

fn gpio_call(pin: u32, value: u32, reg: u32, bits: u32) {
    let field_mask = (1 << bits) - 1 as u32;
    let num_fields = 32 / bits;
    let reg = reg + pin / num_fields * 4;
    let offset = (pin % num_fields) * bits;

    unsafe {
        let mut curval = read_volatile(reg as *const u32 as *const _);
        curval &= !(field_mask << offset);
        curval |= value << offset;
        write_volatile(reg as *mut u32 as *mut _, curval);
    }
}

const GPIO_BASE: u32 = 0xFE200000;
const GPIO_MAX_PIN: u32 = 57;

pub enum Mode {
    Input = 0b000,
    Output = 0b001,
    Alt0 = 0b100,
    Alt1 = 0b101,
    Alt2 = 0b110,
    Alt3 = 0b111,
    Alt4 = 0b011,
    Alt5 = 0b010,
}
pub fn set_mode(mode: Mode, pin: u32) {
    gpio_call(pin, mode as u32, GPIO_BASE + 0x0, 3);
}

pub enum Value {
    High = 1,
    Low = 0,
}
pub fn set_value(value: Value, pin: u32) {
    match value {
        Value::High => gpio_call(pin, 1, GPIO_BASE + 0x1C, 1),
        Value::Low => gpio_call(pin, 1, GPIO_BASE + 0x28, 1),
    }
}

pub enum Pull {
    None = 0b00,
    Up = 0b01,
    Down = 0b10,
}
pub fn set_pull(pull: Pull, pin: u32) {
    gpio_call(pin, pull as u32, GPIO_BASE + 0xE8, 2);
}
