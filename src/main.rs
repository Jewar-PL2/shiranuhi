use spdlog::prelude::*;
// use bitfield_struct::bitfield;

// #[derive(Debug, PartialEq, Eq)]
// #[repr(u8)]
// enum StatusMode {
//     Kernel = 0,
//     User = 1
// }
// impl StatusMode {
//     const fn into_bits(self) -> u8 { self as _ }
//     const fn from_bits(value: u8) -> Self {
//         match value {
//             0 => Self::Kernel,
//             1 => Self::User,
//             _ => unreachable!()
//         }
//     }
// }

// #[derive(Debug, PartialEq, Eq)]
// #[repr(u8)]
// enum StatusBootExceptionVectors {
//     RAM = 0,
//     ROM = 1
// }
// impl StatusBootExceptionVectors {
//     const fn into_bits(self) -> u8 { self as _ }
//     const fn from_bits(value: u8) -> Self {
//         match value {
//             0 => Self::RAM,
//             1 => Self::ROM,
//             _ => unreachable!()
//         }
//     }
// }

// #[bitfield(u32)]
// struct StatusRegister {
//     interrupt_enable: bool,
//     #[bits(1)]
//     mode: StatusMode,
//     previous_interrupt_enable: bool,
//     #[bits(1)]
//     previous_mode: StatusMode,
//     old_interrupt_enable: bool,
//     #[bits(1)]
//     old_mode: StatusMode,
//     #[bits(2)]
//     _pad1: usize,
//     interrupt_mask: u8,
//     isolate_cache: bool,
//     swapped_cache: bool,
//     write_zero_as_parity_bits: bool,
//     #[bits(1)]
//     _pad2: usize,
//     cache_parity_error: bool,
//     tlb_shutdown: bool,
//     #[bits(1)]
//     boot_exception_vectors: StatusBootExceptionVectors,
//     #[bits(2)]
//     _pad3: usize,
//     reverse_endianness: bool,
//     #[bits(2)]
//     _pad4: usize,
//     cop0_enable: bool,
//     cop1_enable: bool,
//     cop2_enable: bool,
//     cop3_enable: bool
// }

fn main() {
    info!("spdlog works fine");

    // let mut status = StatusRegister::new();
    // status.set_isolate_cache(true);
    // let raw_status: u32 = status.into();

    // info!("Status register: 0x{raw_status:08X}");
}
