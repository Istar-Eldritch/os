use register::*;

#[register(txdata, TXDATA, 0x0)]
#[register(rxdata, RXDATA, 0x4)]
#[register(txctrl, TXCTRL, 0x08)]
#[register(rxctrl, RXCTRL, 0x0C)]
#[register(ie, InterruptRegister, 0x10)]
#[register(ip, InterruptRegister, 0x14)]
#[register(div, DIV, 0x18)]
#[derive(Clone)]
pub struct UART(*mut usize);

impl UART {
    pub fn new(addr: *mut usize) -> Self {
        UART(addr)
    }
}

#[field(data, 0, 7)]
#[field(full, 31, 31)]
pub struct TXDATA(*mut usize);


#[field(data, 0, 7)]
#[field[empty, 31, 31]]
pub struct RXDATA(*mut usize);


#[field(txen, 0, 0)]
#[field(nxtop, 1, 1)]
#[field(txcnt, 16, 18)]
pub struct TXCTRL(*mut usize);



#[field(rxen, 0, 0)]
#[field(rxcnt, 16, 18)]
pub struct RXCTRL(*mut usize);



#[field(txwm, 0, 0)]
#[field(rxwm, 1, 1)]
pub struct InterruptRegister(*mut usize);



#[field(div, 0, 15)]
pub struct DIV(*mut usize);


