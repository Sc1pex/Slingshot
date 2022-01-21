use core::ptr::{read_volatile, write_volatile};

use crate::sync::{Mutex, NullLock};

use super::DeviceDriver;

#[allow(dead_code)]
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
#[allow(dead_code)]
pub enum Value {
    High = 1,
    Low = 0,
}
#[allow(dead_code)]
pub enum Pull {
    None = 0b00,
    Up = 0b01,
    Down = 0b10,
}

pub struct GPIOInner {
    base: u32,
}

impl GPIOInner {
    pub const fn new() -> GPIOInner {
        GPIOInner { base: 0xFE200000 }
    }

    fn gpio_call(&self, pin: u32, value: u32, reg: u32, bits: u32) {
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

    pub fn set_mode(&self, pin: u32, mode: Mode) {
        self.gpio_call(pin, mode as u32, self.base, 3);
    }

    pub fn set_value(&self, pin: u32, value: Value) {
        match value {
            Value::High => self.gpio_call(pin, 1, self.base + 0x1C, 1),
            Value::Low => self.gpio_call(pin, 1, self.base + 0x28, 1),
        }
    }

    pub fn set_pull(&self, pin: u32, pull: Pull) {
        self.gpio_call(pin, pull as u32, self.base + 0xE8, 2);
    }
}

pub struct GPIODriver {
    inner: NullLock<GPIOInner>,
}

impl GPIODriver {
    pub const fn new() -> GPIODriver {
        GPIODriver {
            inner: NullLock::new(GPIOInner::new()),
        }
    }

    pub fn set_mode(&self, pin: u32, mode: Mode) {
        self.inner.lock(|inner| inner.set_mode(pin, mode));
    }

    pub fn set_value(&self, pin: u32, value: Value) {
        self.inner.lock(|inner| inner.set_value(pin, value));
    }

    pub fn set_pull(&self, pin: u32, pull: Pull) {
        self.inner.lock(|inner| inner.set_pull(pin, pull));
    }

    pub fn map_uart(&self) {
        self.set_mode(14, Mode::Alt0);
        self.set_mode(15, Mode::Alt0);

        self.set_pull(14, Pull::Up);
        self.set_pull(15, Pull::Down);
    }
}

impl DeviceDriver for GPIODriver {
    fn name(&self) -> &'static str {
        "GPIO"
    }

    fn init(&self) {}

    fn post_init(&self) {
        self.map_uart();
    }
}
