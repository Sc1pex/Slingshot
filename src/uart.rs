use crate::gpio;
use core::ptr::{read_volatile, write_volatile};

const UART_BASE: u32 = crate::gpio::PERIPHERAL_BASE + 0x201000;
const UART_DR: u32 = UART_BASE + 0x00;
const UART_FR: u32 = UART_BASE + 0x18;
const UART_IBRD: u32 = UART_BASE + 0x24;
const UART_FBRD: u32 = UART_BASE + 0x28;
const UART_LCRH: u32 = UART_BASE + 0x2c;
const UART_CR: u32 = UART_BASE + 0x30;
const UART_ICR: u32 = UART_BASE + 0x44;

pub fn init() {
    // Setup gpio pins
    gpio::setup_pin(14, gpio::Mode::Alt0, gpio::Pull::Up);
    gpio::setup_pin(15, gpio::Mode::Alt0, gpio::Pull::Up);
    unsafe {
        // Turn of uart
        write_volatile(UART_CR as *mut u32, 0);
        write_volatile(UART_ICR as *mut u32, 0);
        // Set baudrate
        // 48 Mhz clock
        // 48000000 / 16 / 115200 = 26.04166667 so IBRD is 26
        // and FBRD = INT((.04166667 * 64) + 0.5) = 3
        // This results in 48000000 / (16 * (26 + 3/64)) = 115177
        // Error = (115200 - 115177) / 115200 * 100 = 0.02%
        write_volatile(UART_IBRD as *mut u32, 26);
        write_volatile(UART_FBRD as *mut u32, 3);
        // Set 8 bits and enable fifo
        write_volatile(UART_LCRH as *mut u32, 0b111 << 4);
        // Turn uart on
        write_volatile(UART_CR as *mut u32, 1 | (0b11 << 8));
    }
}

pub fn write_char(c: char) {
    if c == '\n' {
        write_char('\r');
    }

    unsafe {
        // Wait until fifo is not full
        while (read_volatile(UART_FR as *const u32) & (1 << 5)) != 0 {
            core::arch::asm!("nop");
        }
        // Write character
        write_volatile(UART_DR as *mut u32, c as u32);
    }
}

pub fn write_string(s: &'static str) {
    for c in s.chars() {
        write_char(c);
    }
}

pub fn read_char() -> char {
    unsafe {
        while (read_volatile(UART_FR as *const u32) & (1 << 4)) != 0 {
            core::arch::asm!("nop");
        }

        let mut c = read_volatile(UART_DR as *const u32) as u8 as char;
        if c == '\r' {
            c = '\n';
        }

        c
    }
}
