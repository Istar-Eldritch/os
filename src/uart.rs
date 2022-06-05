#![allow(dead_code)]

use crate::bit;
use crate::drivers::gpio::*;
use crate::hifive::*;
use core::ptr::{read_volatile, write_volatile};

const UART0_ADDR: u32 = 0x10_013_000;
const UART0_PIN_RX: u32 = bit!(16);
const UART0_PIN_TX: u32 = bit!(17);

//  const UART1_ADDR: u32 = 0x10_023_000;
//  const UART1_PIN_RX: u32 = bit!(18);
//  const UART1_PIN_TX: u32 = bit!(23);

pub const UART_TXDATA: u32 = 0x00;
pub const UART_RXDATA: u32 = 0x04;
pub const UART_TXCTRL: u32 = 0x08;
pub const UART_RXCTRL: u32 = 0x0c;
pub const UART_IE: u32 = 0x10;
pub const UART_IP: u32 = 0x14;
pub const UART_DIV: u32 = 0x18;

// TIMER STUFF
// TODO: Link docs
pub const PRCI_ADDR: *mut u32 = 0x10008000 as *mut u32;

pub const OTP_ADDR: u32 = 0x20000;

pub const OTP_A: u32 = 0x28;
pub const OTP_Q: u32 = 0x30;

pub const OTP_TRIM: u32 = 0x7fb;

//const FAC_CAL_ADDR: u32 = 2043;

pub fn setup_clock() {
    unsafe {
        // Why this number? Reference docs
        // I couldn't use OTP to find the calibration value so I seeked and found it by trial and
        // error.
        // hfrosctrim is part of the PRCI hfrosccfg register: Ring Oscillator Configuration and Status
        //
        //loop {
        //    write_volatile(OTP_ADDR as *mut u32, 1);
        //    if read_volatile(OTP_ADDR as *mut u32) == 1 {
        //        break;
        //    }
        //}
        //write_volatile((OTP_ADDR + OTP_A) as *mut u32, FAC_CAL_ADDR);
        //let clock_calibration = read_volatile((OTP_ADDR + OTP_Q) as *mut u32);

        // Marks all but the clock divider & calibration sections;
        let mask = !(0b11111 << 16 | 0b11111);

        // TODO: Calibration should be read from the OTP
        let clock_calibration = 0x24 << 16;
        let clock_divider = 0x4;
        write_volatile(
            PRCI_ADDR,
            // Clears the clock divider and calibration sections and sets them
            (read_volatile(PRCI_ADDR) & mask) | clock_divider | clock_calibration,
        );
    }
}

pub fn setup_uart0() {
    // Enble UART GPIOs
    let mut gpio = GPIO::new(GPIO_ADDR);
    gpio.set_iof_enabled(UART0_PIN_TX | UART0_PIN_RX);
    gpio.set_iof_selection(0x0);

    unsafe {
        // Enable UARTs
        write_volatile((UART0_ADDR + UART_TXCTRL) as *mut u32, 1);
        write_volatile((UART0_ADDR + UART_RXCTRL) as *mut u32, 1);

        // Divider to achieve 115200 baud rate at 14.4Mhz
        write_volatile((UART0_ADDR + UART_DIV) as *mut u32, 124)
    }
}

pub struct Writer {
    data: *mut u32,
}

impl Writer {
    pub fn new() -> Self {
        Writer {
            data: (UART0_ADDR + UART_TXDATA) as *mut u32,
        }
    }
    pub fn write_char(&mut self, c: char) {
        unsafe {
            loop {
                if read_volatile(self.data) & bit!(31) == 0 {
                    break;
                }
            }
            write_volatile(self.data, c as u32)
        }
    }

    pub fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_char(byte as char);
        }
    }
}
