use crate::devices::Devices;
use crate::drivers::{clint::*, plic::*};
use crate::hifive::*;
use crate::println;
use crate::riscv::*;

static mut TRAP_MANAGER: Option<TrapManager> = None;

#[derive(Debug)]
#[allow(dead_code)]
pub struct InterruptEvent {
    time: u64,
    exception: bool,
    code: usize,
    pc: usize,
    val: usize,
}

pub type TrapHandler = fn(e: &InterruptEvent) -> ();

pub enum InterruptCode {
    MachineSoftwareInterrupt = 3,
    MachineTimerInterrupt = 7,
    MachineExternalInterrupt = 11,
}

impl From<usize> for InterruptCode {
    fn from(v: usize) -> Self {
        use InterruptCode::*;
        match v {
            3 => MachineSoftwareInterrupt,
            7 => MachineTimerInterrupt,
            11 => MachineExternalInterrupt,
            _ => panic!("Value not supported"),
        }
    }
}

pub enum ExceptionCode {
    InstructionAddressMisaligned = 0,
    InstructionAccessFault = 1,
    IllegalInstruction = 2,
    Breakpoint = 3,
    LoadAddressMisaligned = 4,
    LoadAccessFault = 5,
    StoreAddressMisaligned = 6,
    StoreAccessFault = 7,
    EnvironmentalCallFromUMode = 8,
    EnvironmentalCallFromMMode = 11,
}

impl From<usize> for ExceptionCode {
    fn from(v: usize) -> Self {
        use ExceptionCode::*;
        match v {
            0 => InstructionAddressMisaligned,
            1 => InstructionAccessFault,
            2 => IllegalInstruction,
            3 => Breakpoint,
            4 => LoadAddressMisaligned,
            5 => LoadAccessFault,
            6 => StoreAddressMisaligned,
            7 => StoreAccessFault,
            8 => EnvironmentalCallFromUMode,
            11 => EnvironmentalCallFromMMode,
            _ => panic!("Value not supported"),
        }
    }
}

pub struct TrapManager {
    interrupt_handler: [Option<TrapHandler>; 12],
    exception_handler: [Option<TrapHandler>; 12],
    external_interrupt_handler: [Option<TrapHandler>; 53],
}

impl TrapManager {
    pub fn register_interrupt_handler(&mut self, code: InterruptCode, handler: TrapHandler) {
        self.interrupt_handler[code as usize] = Some(handler);
    }

    //TODO: Should we use the trap manager only for interrupts?
    #[allow(dead_code)]
    pub fn register_exception_handler(&mut self, code: ExceptionCode, handler: TrapHandler) {
        self.exception_handler[code as usize] = Some(handler);
    }

    pub fn register_external_interrupt_handler(&mut self, code: u8, handler: TrapHandler) {
        self.external_interrupt_handler[code as usize] = Some(handler)
    }

    pub fn get_interrupt_handler(&self, code: InterruptCode) -> &Option<TrapHandler> {
        &self.interrupt_handler[code as usize]
    }

    pub fn get_exception_handler(&self, code: ExceptionCode) -> &Option<TrapHandler> {
        &self.exception_handler[code as usize]
    }

    pub fn get_external_interrupt_handler(&self, code: u8) -> &Option<TrapHandler> {
        &self.external_interrupt_handler[code as usize]
    }

    pub fn get<'a>() -> &'a Self {
        unsafe { TRAP_MANAGER.as_ref() }.unwrap()
    }

    pub fn get_mut<'a>() -> &'a mut Self {
        unsafe { TRAP_MANAGER.as_mut() }.unwrap()
    }

    pub unsafe fn init() {
        let mut trap_manager = TrapManager {
            interrupt_handler: [None; 12],
            exception_handler: [None; 12],
            external_interrupt_handler: [None; 53],
        };

        trap_manager.register_interrupt_handler(InterruptCode::MachineExternalInterrupt, |v| {
            let plic = Plic::new(PLIC_ADDR);
            let external_interrupt = plic.claim().all() as u8;
            if let Some(handler) =
                TrapManager::get().get_external_interrupt_handler(external_interrupt)
            {
                handler(v);
            } else {
                println!(
                    "\n\rExternal interrupt {} not handled: {:?}",
                    external_interrupt, v
                );
            }
        });

        TRAP_MANAGER = Some(trap_manager)
    }
}

#[no_mangle]
pub fn trap_handler() {
    let mcause = MCause::new();
    let clint = Clint::new(CLINT_ADDR);
    let time: u64 = clint.mtime().get_time();

    let i = InterruptEvent {
        time,
        exception: mcause.interrupt() != 1,
        code: mcause.code(),
        pc: Mepc::new().all(),
        val: Mtval::new().all(),
    };

    if mcause.interrupt() == 1 {
        let code: InterruptCode = mcause.code().into();
        if let Some(handler) = TrapManager::get().get_interrupt_handler(code) {
            handler(&i)
        } else {
            println!("\n\rInterrupt Not handled: {:?}", i);
        }
    } else {
        let code: ExceptionCode = mcause.code().into();
        if let Some(handler) = TrapManager::get().get_exception_handler(code) {
            handler(&i)
        } else {
            let d = Devices::get();
            d.leds.set_green(false);
            d.leds.set_blue(false);
            d.leds.set_red(true);
            println!("\n\rException not handled: {:?}\n\rHALTED!", i);
            // TODO HALT / Recover
            loop {}
        }
    }
}
