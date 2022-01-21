use core::ptr::{read_volatile, write_volatile};

use crate::gpio;

const UART_BASE: u32 = 0xFE201000;
const UART_DR: u32 = UART_BASE + 0x00;
const UART_FR: u32 = UART_BASE + 0x18;
const UART_IBRD: u32 = UART_BASE + 0x24;
const UART_FBRD: u32 = UART_BASE + 0x28;
const UART_LCHR: u32 = UART_BASE + 0x02C;
const UART_CR: u32 = UART_BASE + 0x30;
const UART_ICR: u32 = UART_BASE + 0x44;

pub fn init() {
    unsafe {
        write_volatile(UART_CR as *mut u32 as *mut _, 0);
        write_volatile(UART_ICR as *mut u32 as *mut _, 0);

        write_volatile(UART_IBRD as *mut u32 as *mut _, 26);
        write_volatile(UART_FBRD as *mut u32 as *mut _, 3);

        write_volatile(UART_LCHR as *mut u32 as *mut _, 111 << 4);

        write_volatile(UART_CR as *mut u32 as *mut _, 1 | (11 << 8));
    }

    gpio::map_uart();
}

pub fn write_char(c: char) {
    unsafe {
        while (read_volatile(UART_FR as *const u32 as *const _) & ((1 << 5) as u32)) > 0 as u32 {
            asm!("nop");
        }

        write_volatile(UART_DR as *mut u32 as *mut _, c);
    }
}

pub fn write_str(s: &str) {
    for c in s.chars() {
        write_char(c);
    }
}
