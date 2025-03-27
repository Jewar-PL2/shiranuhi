use bitfield_struct::bitfield;
use spdlog::prelude::*;

pub struct Cop0 {
    status: Status,
    cause: Cause
}

impl Cop0 {
    pub fn new() -> Self {
        Self {
            status: Status::new(),
            cause: Cause::new()
        }
    }

    pub fn is_cache_isolated(&self) -> bool {
        self.status.isolate_cache()
    }

    pub fn load(&self, register: usize) -> Option<u32> {
        trace!("Reading from COP0 register: {}", register);
        
        match register {
            12 => Some(self.status.into()),
            13 => Some(self.cause.into()),
            _ => None
        }
    }

    pub fn store(&mut self, register: usize, value: u32) {
        trace!("Writing to COP0 register: {}, value: 0x{:08X}", register, value);

        match register {
            // TODO: Implement these registers
            3 | 5 | 6 | 7 | 9 | 11 => (),
            12 => self.status.0 = value,
            13 => {
                // Does it work properly?
                let mut interrupt_pending = self.cause.interrupt_pending();
                interrupt_pending &= 3;
                interrupt_pending |= (value & 0x300) >> 8;
                self.cause.set_interrupt_pending(interrupt_pending);
            }
            _ => unimplemented!()
        }
    }
}

#[repr(u32)]
#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Kernel = 0,
    User = 1
}
impl Mode {
    const fn into_bits(self) -> u32 { self as _ }
    const fn from_bits(value: u32) -> Self {
        match value {
            0 => Self::Kernel,
            1 => Self::User,
            _ => unreachable!()
        }
    }
}

#[repr(u32)]
#[derive(Debug, PartialEq, Eq)]
pub enum BootExceptionVectors {
    KSEG0 = 0,
    KSEG1 = 1
}
impl BootExceptionVectors {
    const fn into_bits(self) -> u32 { self as _ }
    const fn from_bits(value: u32) -> Self {
        match value {
            0 => Self::KSEG0,
            1 => Self::KSEG1,
            _ => unreachable!()
        }
    }
}

#[bitfield(u32)]
pub struct Status {
    interrupt_enable: bool,
    #[bits(1)]
    mode: Mode,

    prev_interrupt_enable: bool,
    #[bits(1)]
    prev_mode: Mode,

    old_interrupt_enable: bool,
    #[bits(1)]
    old_mode: Mode,

    #[bits(2)]
    _pad1: u32,

    interrupt_mask: u8,
    isolate_cache: bool,
    swapped_cache: bool,
    write_zero_as_parity_bits: bool,
    _pad2: bool,
    cache_parity_error: bool,
    tlb_shutdown: bool,

    #[bits(1)]
    boot_exception_vectors: BootExceptionVectors,
    #[bits(2)]
    _pad3: u32,
    reverse_endianness: bool,
    #[bits(2)]
    _pad4: u32,

    cop0_enable: bool,
    cop1_enable: bool,
    cop2_enable: bool,
    cop3_enable: bool,
}

#[repr(u32)]
#[derive(Debug, PartialEq, Eq)]
pub enum Exception {
    Interrupt = 0x00,
    TlbModification = 0x01,
    TlbLoad = 0x02,
    TlbStore = 0x03,
    AddressLoadError = 0x04,
    AddressStoreError = 0x05,
    BusInstructionError = 0x06,
    BusDataLoadStoreError = 0x07,
    SystemCall = 0x08,
    Breakpoint = 0x09,
    ReservedInstruction = 0x0A,
    CoprocessorUnusable = 0x0B,
    ArithmeticOverflow = 0x0C
}
impl Exception {
    const fn into_bits(self) -> u32 { self as _ }
    const fn from_bits(value: u32) -> Self {
        match value {
            0x00 => Self::Interrupt,
            0x01 => Self::TlbModification,
            0x02 => Self::TlbLoad,
            0x03 => Self::TlbStore,
            0x04 => Self::AddressLoadError,
            0x05 => Self::AddressStoreError,
            0x06 => Self::BusInstructionError,
            0x07 => Self::BusDataLoadStoreError,
            0x08 => Self::SystemCall,
            0x09 => Self::Breakpoint,
            0x0A => Self::ReservedInstruction,
            0x0B => Self::CoprocessorUnusable,
            0x0C => Self::ArithmeticOverflow,
            _ => unreachable!()
        }
    }
}

#[bitfield(u32)]
pub struct Cause {
    #[bits(2)]
    _pad1: u32,
    #[bits(5)]
    exception_code: Exception,
    #[bits(1)]
    _pad2: u32,
    #[bits(8)]
    interrupt_pending: u32,
    #[bits(12)]
    _pad3: u32,
    #[bits(2)]
    coprocessor_number: u32,
    branch_taken: bool,
    branch_delay: bool
}
