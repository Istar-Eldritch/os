#![allow(dead_code)]

/// GPIO Module as defined in the HiFive RevB board.
/// TODO: Validate if this is a common standard
use core::ptr::write_volatile;

const GPIO_IN_VAL: u32 = 0x00 as u32;
const GPIO_IN_EN: u32 = 0x04 as u32;
const GPIO_OUT_EN: u32 = 0x08 as u32;
const GPIO_OUT_VAL: u32 = 0x0C as u32;
const GPIO_PUE: u32 = 0x10 as u32;
const GPIO_DS: u32 = 0x14 as u32;
const GPIO_RISE_IE: u32 = 0x18 as u32;
const GPIO_RISE_IP: u32 = 0x1C as u32;
const GPIO_FALL_IE: u32 = 0x20 as u32;
const GPIO_FALL_IP: u32 = 0x24 as u32;
const GPIO_HIGH_IE: u32 = 0x28 as u32;
const GPIO_HIGH_IP: u32 = 0x2C as u32;
const GPIO_LOW_IE: u32 = 0x38 as u32;
const GPIO_LOW_IP: u32 = 0x34 as u32;
const GPIO_IOF_EN: u32 = 0x38 as u32;
const GPIO_IOF_SEL: u32 = 0x3C as u32;
const GPIO_OUT_XOR: u32 = 0x40 as u32;
const GPIO_PASSTHRU_HIGH_IE: u32 = 0x44 as u32;
const GPIO_PASSTHRU_LOW_IE: u32 = 0x48 as u32;

pub struct GPIO {
    base: *mut u32,
}

impl GPIO {
    pub fn new(base: *mut u32) -> Self {
        GPIO { base }
    }

    pub fn set_output_enabled(&mut self, pin: u32) {
        unsafe {
            write_volatile((self.base as u32 + GPIO_OUT_EN) as *mut u32, pin);
            write_volatile((self.base as u32 + GPIO_OUT_XOR) as *mut u32, pin);
        }
    }

    pub fn set_output_value(&mut self, value: u32) {
        unsafe {
            write_volatile((self.base as u32 + GPIO_OUT_VAL) as *mut u32, value);
        }
    }

    pub fn set_iof_enabled(&mut self, pin: u32) {
        unsafe {
            write_volatile((self.base as u32 + GPIO_IOF_EN) as *mut u32, pin);
        }
    }

    pub fn set_iof_selection(&mut self, value: u32) {
        unsafe {
            write_volatile((self.base as u32 + GPIO_IOF_SEL) as *mut u32, value);
        }
    }
}
