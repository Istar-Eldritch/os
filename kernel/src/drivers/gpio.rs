#![allow(dead_code)]

use register::*;


#[register(in_val, PinRegister, 0x00)]
#[register(in_en, PinRegister, 0x04)]
#[register(output_en, PinRegister, 0x08)]
#[register(output_val, PinRegister, 0x0C)]
#[register(pue, PinRegister, 0x10)]
#[register(ds, PinRegister, 0x14)]
#[register(rise_ie, PinRegister, 0x18)]
#[register(rise_ip, PinRegister, 0x1C)]
#[register(fall_ie, PinRegister, 0x20)]
#[register(fall_ip, PinRegister, 0x24)]
#[register(high_ie, PinRegister, 0x28)]
#[register(high_ip, PinRegister, 0x2C)]
#[register(low_ie, PinRegister, 0x38)]
#[register(low_ip, PinRegister, 0x34)]
#[register(iof_en, PinRegister, 0x38)]
#[register(iof_sel, PinRegister, 0x3C)]
#[register(out_xor, PinRegister, 0x40)]
#[register(passthru_high_ie, PinRegister, 0x44)]
#[register(passthru_low_ie, PinRegister, 0x48)]
pub struct Gpio {
    addr: *mut usize,
}

impl Gpio {
    pub fn new(addr: *mut usize) -> Self {
        Gpio { addr }
    }
}

#[field(pin0, 0, 0)]
#[field(pin1, 1, 1)]
#[field(pin2, 2, 2)]
#[field(pin3, 3, 3)]
#[field(pin4, 4, 4)]
#[field(pin5, 5, 5)]
#[field(pin6, 6, 6)]
#[field(pin7, 7, 7)]
#[field(pin8, 8, 8)]
#[field(pin9, 9, 9)]
#[field(pin10, 10, 10)]
#[field(pin11, 11, 11)]
#[field(pin12, 12, 12)]
#[field(pin13, 13, 13)]
#[field(pin14, 14, 14)]
#[field(pin15, 15, 15)]
#[field(pin16, 16, 16)]
#[field(pin17, 17, 17)]
#[field(pin18, 18, 18)]
#[field(pin19, 19, 19)]
#[field(pin20, 20, 20)]
#[field(pin21, 21, 21)]
#[field(pin22, 22, 22)]
#[field(pin23, 23, 23)]
#[field(pin24, 24, 24)]
#[field(pin25, 25, 25)]
#[field(pin26, 26, 26)]
#[field(pin27, 27, 27)]
#[field(pin28, 28, 28)]
#[field(pin29, 29, 29)]
#[field(pin30, 30, 30)]
#[field(pin31, 31, 31)]
#[field(all, 0, 31)]
pub struct PinRegister {
    addr: *mut usize,
}
