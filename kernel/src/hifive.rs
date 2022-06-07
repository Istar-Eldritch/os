#![allow(dead_code)]

/// Hifive specific constants
///
use crate::bit;

pub const GPIO_ADDR: *mut u32 = 0x10_012_000 as *mut u32;

pub const LED_RED: u32 = bit!(22);
pub const LED_GREEN: u32 = bit!(19);
pub const LED_BLUE: u32 = bit!(21);

pub const UART0_ADDR: *mut u32 = 0x10_013_000 as *mut u32;
pub const UART0_PIN_RX: u32 = bit!(16);
pub const UART0_PIN_TX: u32 = bit!(17);

pub const UART1_ADDR: *mut u32 = 0x10_023_000 as *mut u32;
pub const UART1_PIN_RX: u32 = bit!(18);
pub const UART1_PIN_TX: u32 = bit!(23);

// TIMER STUFF
// TODO: Link docs
pub const PRCI_ADDR: *mut u32 = 0x10_008_000 as *mut u32;

// OTP SUFF
pub const OTP_ADDR: u32 = 0x20000;
pub const OTP_A: u32 = 0x28;
pub const OTP_Q: u32 = 0x30;
pub const OTP_TRIM: u32 = 0x7fb;

//const FAC_CAL_ADDR: u32 = 2043;
