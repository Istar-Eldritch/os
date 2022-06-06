#![allow(dead_code)]

use core::ptr::{read_volatile, write_volatile};

pub struct UART {
    pub txdata: TXDATA,
    pub rxdata: RXDATA,
    pub txctrl: TXCTRL,
    pub rxctrl: RXCTRL,
    pub ie: IE,
    pub ip: IP,
    pub div: DIV,
}

impl UART {
    pub fn new(addr: *mut u32) -> Self {
        UART {
            txdata: TXDATA { ptr: addr },
            rxdata: RXDATA {
                ptr: (addr as u32 + 0x04) as *mut u32,
            },
            txctrl: TXCTRL {
                ptr: (addr as u32 + 0x08) as *mut u32,
            },
            rxctrl: RXCTRL {
                ptr: (addr as u32 + 0x0C) as *mut u32,
            },
            ie: IE {
                ptr: (addr as u32 + 0x10) as *mut u32,
            },
            ip: IP {
                ptr: (addr as u32 + 0x14) as *mut u32,
            },
            div: DIV {
                ptr: (addr as u32 + 0x18) as *mut u32,
            },
        }
    }
}

pub struct TXDATA {
    ptr: *mut u32,
}

impl TXDATA {
    pub fn new(ptr: *mut u32) -> Self {
        TXDATA { ptr }
    }

    /// Transmit data
    pub fn data(&self) -> u8 {
        let value = unsafe { read_volatile(self.ptr) & 0xFF };
        value as u8
    }

    pub fn set_data(&mut self, value: u8) {
        unsafe { write_volatile(self.ptr, value as u32) }
    }

    /// Transmit FIFO full
    pub fn full(&self) -> bool {
        unsafe { read_volatile(self.ptr) & (1 << 31) != 0 }
    }
}

pub struct RXDATA {
    ptr: *mut u32,
}

impl RXDATA {
    pub fn new(ptr: *mut u32) -> Self {
        RXDATA { ptr }
    }

    /// Received data
    pub fn data(&self) -> u8 {
        let value = unsafe { read_volatile(self.ptr) & 0xF };
        u8::try_from(value).unwrap()
    }

    /// Receive FIFO empty
    pub fn empty(&self) -> bool {
        unsafe { read_volatile(self.ptr) & (1 << 31) != 1 }
    }
}

pub struct TXCTRL {
    ptr: *mut u32,
}

impl TXCTRL {
    pub fn new(ptr: *mut u32) -> Self {
        TXCTRL { ptr }
    }

    /// Transmit enable
    pub fn txen(&self) -> bool {
        unsafe { read_volatile(self.ptr) & 0x1 != 0 }
    }

    pub fn set_txen(&mut self, value: bool) {
        let value = if value { 1 } else { 0 };
        unsafe { write_volatile(self.ptr, !(read_volatile(self.ptr) & 0x1) | value) }
    }

    pub fn nstop(&self) -> bool {
        unsafe { read_volatile(self.ptr) & (0x1 << 1) != 0 }
    }

    pub fn set_nstop(&mut self, value: bool) {
        let value = if value { 1 << 1 } else { 0 };
        unsafe {
            write_volatile(
                self.ptr,
                (read_volatile(self.ptr) & !(0x1 << 1)) | (value << 1),
            )
        }
    }

    pub fn txcnt(&self) -> u8 {
        let val = unsafe { read_volatile(self.ptr) & (0b11 << 16) };
        (val >> 16) as u8
    }

    pub fn set_txcnt(&mut self, value: u8) {
        unsafe {
            write_volatile(
                self.ptr,
                (read_volatile(self.ptr) & !(0b11 << 16)) | (value as u32) << 16,
            )
        }
    }
}

pub struct RXCTRL {
    ptr: *mut u32,
}

impl RXCTRL {
    pub fn new(ptr: *mut u32) -> Self {
        RXCTRL { ptr }
    }

    pub fn rxen(&self) -> bool {
        unsafe { read_volatile(self.ptr) & 0x1 != 0 }
    }

    pub fn set_rxen(&mut self, value: bool) {
        let value = if value { 1 } else { 0 };
        unsafe { write_volatile(self.ptr, !(read_volatile(self.ptr) & 0x1) | value) }
    }

    pub fn rxcnt(&self) -> u8 {
        let val = unsafe { read_volatile(self.ptr) & (0b11 << 16) };
        (val >> 16) as u8
    }

    pub fn set_rxcnt(&mut self, value: u8) {
        unsafe {
            write_volatile(
                self.ptr,
                (read_volatile(self.ptr) & !(0b11 << 16)) | (value as u32) << 16,
            )
        }
    }
}

pub struct IE {
    ptr: *mut u32,
}

impl IE {
    pub fn new(ptr: *mut u32) -> Self {
        IE { ptr }
    }

    pub fn txwm(&self) -> bool {
        unsafe { read_volatile(self.ptr) & 0x1 != 0 }
    }

    pub fn set_txwm(&mut self, value: bool) {
        let value = if value { 1 } else { 0 };
        unsafe { write_volatile(self.ptr, !(read_volatile(self.ptr) & 0x1) | value) }
    }

    pub fn rxwm(&self) -> bool {
        unsafe { read_volatile(self.ptr) & (0x1 << 1) != 0 }
    }

    pub fn set_rxwm(&mut self, value: bool) {
        let value = if value { 1 } else { 0 };
        unsafe { write_volatile(self.ptr, !(read_volatile(self.ptr) & (0x1 << 1)) | value) }
    }
}

pub struct IP {
    ptr: *mut u32,
}

impl IP {
    pub fn new(ptr: *mut u32) -> Self {
        IP { ptr }
    }

    pub fn txwm(&self) -> bool {
        unsafe { read_volatile(self.ptr) & 0x1 != 0 }
    }

    pub fn rxwm(&self) -> bool {
        unsafe { read_volatile(self.ptr) & (0x1 << 1) != 0 }
    }
}

pub struct DIV {
    ptr: *mut u32,
}

impl DIV {
    pub fn new(ptr: *mut u32) -> Self {
        DIV { ptr }
    }

    pub fn div(&self) -> u16 {
        let val = unsafe { read_volatile(self.ptr) & 0xFFFF };
        val as u16
    }

    pub fn set_div(&self, value: u16) {
        unsafe {
            let original = read_volatile(self.ptr) & !0xFFFF;
            write_volatile(self.ptr, original | value as u32)
        }
    }
}
