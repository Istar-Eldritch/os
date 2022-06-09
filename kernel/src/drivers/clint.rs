#![allow(dead_code)]

use register::*;

#[register(msip, MSIP, 0)]
#[register(mtimecmp, MTime, 0x4000)]
#[register(mtime, MTime, 0xbff8)]
pub struct CLINT(*mut usize);

#[field(all, 0, 31)]
pub struct MSIP(*mut usize);

#[register(low, Time, 0)]
#[register(high, Time, 0x4)]
pub struct MTime(*mut usize);


#[field(all, 0, 31)]
pub struct Time(*mut usize);
