#![allow(dead_code)]

use core::ptr::{read_volatile, write_volatile};

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
    ptr: *mut u32,
}

impl GPIO {
    pub fn new(ptr: *mut u32) -> Self {
        GPIO { ptr }
    }

    pub fn input_val(&self) -> u32 {
        unsafe { read_volatile((self.ptr as u32 + GPIO_IN_VAL) as *mut u32) }
    }

    pub fn set_input_val(&mut self, value: u32) {
        unsafe { write_volatile((self.ptr as u32 + GPIO_IN_VAL) as *mut u32, value) }
    }

    pub fn input_en(&self) -> u32 {
        unsafe { read_volatile((self.ptr as u32 + GPIO_IN_EN) as *mut u32) }
    }

    pub fn set_input_en(&mut self, value: u32) {
        unsafe { write_volatile((self.ptr as u32 + GPIO_IN_EN) as *mut u32, value) }
    }

    pub fn output_en(&self) -> u32 {
        unsafe { read_volatile((self.ptr as u32 + GPIO_OUT_EN) as *mut u32) }
    }

    pub fn set_output_en(&mut self, value: u32) {
        unsafe { write_volatile((self.ptr as u32 + GPIO_OUT_EN) as *mut u32, value) }
    }

    pub fn output_val(&self) -> u32 {
        unsafe { read_volatile((self.ptr as u32 + GPIO_OUT_VAL) as *mut u32) }
    }

    pub fn set_output_val(&mut self, value: u32) {
        unsafe { write_volatile((self.ptr as u32 + GPIO_OUT_VAL) as *mut u32, value) }
    }

    pub fn pue(&self) -> u32 {
        unsafe { read_volatile((self.ptr as u32 + GPIO_PUE) as *mut u32) }
    }

    pub fn set_pue(&mut self, value: u32) {
        unsafe { write_volatile((self.ptr as u32 + GPIO_PUE) as *mut u32, value) }
    }

    pub fn ds(&self) -> u32 {
        unsafe { read_volatile((self.ptr as u32 + GPIO_DS) as *mut u32) }
    }

    pub fn set_ds(&mut self, value: u32) {
        unsafe { write_volatile((self.ptr as u32 + GPIO_DS) as *mut u32, value) }
    }

    pub fn rise_ie(&self) -> u32 {
        unsafe { read_volatile((self.ptr as u32 + GPIO_RISE_IE) as *mut u32) }
    }

    pub fn set_rise_ie(&mut self, value: u32) {
        unsafe { write_volatile((self.ptr as u32 + GPIO_RISE_IE) as *mut u32, value) }
    }

    pub fn rise_ip(&self) -> u32 {
        unsafe { read_volatile((self.ptr as u32 + GPIO_RISE_IP) as *mut u32) }
    }

    pub fn set_rise_ip(&mut self, value: u32) {
        unsafe { write_volatile((self.ptr as u32 + GPIO_RISE_IP) as *mut u32, value) }
    }

    pub fn fall_ie(&self) -> u32 {
        unsafe { read_volatile((self.ptr as u32 + GPIO_FALL_IE) as *mut u32) }
    }

    pub fn set_fall_ie(&mut self, value: u32) {
        unsafe { write_volatile((self.ptr as u32 + GPIO_FALL_IE) as *mut u32, value) }
    }

    pub fn fall_ip(&self) -> u32 {
        unsafe { read_volatile((self.ptr as u32 + GPIO_FALL_IP) as *mut u32) }
    }

    pub fn set_fall_ip(&mut self, value: u32) {
        unsafe { write_volatile((self.ptr as u32 + GPIO_FALL_IP) as *mut u32, value) }
    }

    pub fn high_ie(&self) -> u32 {
        unsafe { read_volatile((self.ptr as u32 + GPIO_HIGH_IE) as *mut u32) }
    }

    pub fn set_high_ie(&mut self, value: u32) {
        unsafe { write_volatile((self.ptr as u32 + GPIO_HIGH_IE) as *mut u32, value) }
    }

    pub fn high_ip(&self) -> u32 {
        unsafe { read_volatile((self.ptr as u32 + GPIO_HIGH_IP) as *mut u32) }
    }

    pub fn set_high_ip(&mut self, value: u32) {
        unsafe { write_volatile((self.ptr as u32 + GPIO_HIGH_IP) as *mut u32, value) }
    }

    pub fn low_ie(&self) -> u32 {
        unsafe { read_volatile((self.ptr as u32 + GPIO_LOW_IE) as *mut u32) }
    }

    pub fn set_low_ie(&mut self, value: u32) {
        unsafe { write_volatile((self.ptr as u32 + GPIO_LOW_IE) as *mut u32, value) }
    }

    pub fn low_ip(&self) -> u32 {
        unsafe { read_volatile((self.ptr as u32 + GPIO_LOW_IP) as *mut u32) }
    }

    pub fn set_low_ip(&mut self, value: u32) {
        unsafe { write_volatile((self.ptr as u32 + GPIO_LOW_IP) as *mut u32, value) }
    }

    pub fn iof_en(&self) -> u32 {
        unsafe { read_volatile((self.ptr as u32 + GPIO_IOF_EN) as *mut u32) }
    }

    pub fn set_iof_en(&mut self, value: u32) {
        unsafe { write_volatile((self.ptr as u32 + GPIO_IOF_EN) as *mut u32, value) }
    }

    pub fn iof_sel(&self) -> u32 {
        unsafe { read_volatile((self.ptr as u32 + GPIO_IOF_SEL) as *mut u32) }
    }

    pub fn set_iof_sel(&mut self, value: u32) {
        unsafe { write_volatile((self.ptr as u32 + GPIO_IOF_SEL) as *mut u32, value) }
    }

    pub fn out_xor(&self) -> u32 {
        unsafe { read_volatile((self.ptr as u32 + GPIO_OUT_XOR) as *mut u32) }
    }

    pub fn set_out_xor(&mut self, value: u32) {
        unsafe { write_volatile((self.ptr as u32 + GPIO_OUT_XOR) as *mut u32, value) }
    }

    pub fn passthru_high_ie(&self) -> u32 {
        unsafe { read_volatile((self.ptr as u32 + GPIO_PASSTHRU_HIGH_IE) as *mut u32) }
    }

    pub fn set_passthru_high_ie(&mut self, value: u32) {
        unsafe { write_volatile((self.ptr as u32 + GPIO_PASSTHRU_HIGH_IE) as *mut u32, value) }
    }

    pub fn passthru_low_ie(&self) -> u32 {
        unsafe { read_volatile((self.ptr as u32 + GPIO_PASSTHRU_LOW_IE) as *mut u32) }
    }

    pub fn set_passthru_low_ie(&mut self, value: u32) {
        unsafe { write_volatile((self.ptr as u32 + GPIO_PASSTHRU_LOW_IE) as *mut u32, value) }
    }
}
