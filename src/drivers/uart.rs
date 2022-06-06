#![allow(dead_code)]

use crate::drivers::gpio::*;
use crate::hifive::*;
use core::ptr::write_volatile;

pub const UART_TXDATA: u32 = 0x00;
pub const UART_RXDATA: u32 = 0x04;
pub const UART_TXCTRL: u32 = 0x08;
pub const UART_RXCTRL: u32 = 0x0c;
pub const UART_IE: u32 = 0x10;
pub const UART_IP: u32 = 0x14;
pub const UART_DIV: u32 = 0x18;

pub fn setup_uart0() {
    // Enble UART GPIOs
    let mut gpio = GPIO::new(GPIO_ADDR);
    gpio.set_iof_enabled(UART0_PIN_TX | UART0_PIN_RX);
    gpio.set_iof_selection(0x0);

    unsafe {
        // Enable UARTs
        write_volatile((UART0_ADDR as u32 + UART_TXCTRL) as *mut u32, 1);
        write_volatile((UART0_ADDR as u32 + UART_RXCTRL) as *mut u32, 1);

        // Divider to achieve 115200 baud rate at 14.4MHz
        write_volatile((UART0_ADDR as u32 + UART_DIV) as *mut u32, 124)
    }
}
