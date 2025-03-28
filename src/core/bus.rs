use super::{
    bios::Bios,
    devices::ram::Ram
};
use spdlog::prelude::*;

const RAM_RANGE: Range = Range(0x00000000, 2 * 1024 * 1024);
const EXPANSION_1_RANGE: Range = Range(0x1F000000, 512 * 1024); // 512KB i think
const MEMORY_CONTROL_RANGE: Range = Range(0x1F801000, 36);
const RAM_SIZE_RANGE: Range = Range(0x1F801060, 4);
const SPU_RANGE: Range = Range(0x1F801C00, 640);
const EXPANSION_2_RANGE: Range = Range(0x1F802000, 66);
const BIOS_RANGE: Range = Range(0x1FC00000, 512 * 1024);
const CACHE_CONTROL_RANGE: Range = Range(0xFFFE0130, 4);

pub struct Bus {
    bios: Bios,
    ram: Ram,
    cache_control: CacheControl
}

impl Bus {
    pub fn new(bios: Bios) -> Self {
        Self { bios, ram: Ram::new(), cache_control: CacheControl(0) }
    }

    pub fn load32(&self, address: u32) -> u32 {        
        let address = get_masked_address(address);
        
        if let Some(offset) = RAM_RANGE.contains(address) {
            return self.ram.load32(offset);
        }

        if let Some(offset) = BIOS_RANGE.contains(address) {
            return self.bios.load32(offset);
        }

        if let Some(_offset) = CACHE_CONTROL_RANGE.contains(address) {
            return self.cache_control.0;
        }

        panic!("INVALID LOAD32 ADDRESS: 0x{:08X}", address);
    }

    pub fn load16(&self, address: u32) -> u16 {        
        let address = get_masked_address(address);

        panic!("INVALID LOAD16 ADDRESS: 0x{:08X}", address);
    }

    pub fn load8(&self, address: u32) -> u8 {        
        let address = get_masked_address(address);

        if let Some(offset) = RAM_RANGE.contains(address) {
            return self.ram.load8(offset);
        }

        if let Some(_offset) = EXPANSION_1_RANGE.contains(address) {
            warn!("[EXP1] Unhandled load8 at [0x{:08X}]", address);
            return 0xFF;
        }

        if let Some(offset) = BIOS_RANGE.contains(address) {
            return self.bios.load8(offset);
        }

        panic!("INVALID LOAD8 ADDRESS: 0x{:08X}", address);
    }

    pub fn store32(&mut self, address: u32, value: u32) {
        let address = get_masked_address(address);

        if let Some(offset) = RAM_RANGE.contains(address) {
            self.ram.store32(offset, value);
            return;
        }

        if let Some(_offset) = MEMORY_CONTROL_RANGE.contains(address) {
            warn!("[MEM_CONTROL] Unhandled store32 at [0x{:08X}]: 0x{:08X}", address, value);
            return;
        }

        if let Some(_offset) = RAM_SIZE_RANGE.contains(address) {
            warn!("[RAM_SIZE] Unhandled store32 at [0x{:08X}]: 0x{:08X}", address, value);
            return;
        }

        if let Some(_offset) = CACHE_CONTROL_RANGE.contains(address) {
            self.cache_control.0 = value;
            return;
        }

        warn!("Unhandled store32 at [0x{:08X}]: 0x{:08X}", address, value)
    }

    pub fn store16(&mut self, address: u32, value: u16) {
        let address = get_masked_address(address);

        if let Some(_offset) = SPU_RANGE.contains(address) {
            warn!("[SPU] Unhandled store16 at [0x{:08X}]: 0x{:04X}", address, value);
            return;
        }

        warn!("Unhandled store16 at [0x{:08X}]: 0x{:04X}", address, value)
    }

    pub fn store8(&mut self, address: u32, value: u8) {
        let address = get_masked_address(address);

        if let Some(_offset) = EXPANSION_2_RANGE.contains(address) {
            warn!("[EXP2] Unhandled store8 at [0x{:08X}]: 0x{:02X}", address, value);
            return;
        }

        warn!("Unhandled store8 at [0x{:08X}]: 0x{:02X}", address, value)
    }
}

pub struct Range(u32, u32);
impl Range {
    pub fn contains(self, address: u32) -> Option<u32> {
        let Range(start, size) = self;

        if address >= start && address < start + size {
            Some(address - start)
        } else {
            None
        }
    }
}

struct CacheControl(u32);
impl CacheControl {
    pub fn icache_enabled(&self) -> bool {
        self.0 & 0x800 != 0
    }

    pub fn tag_test_mode(&self) -> bool {
        self.0 & 0x4 != 0
    }
}

const REGION_MASKS: [u32; 8] = [
    0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, // KUSEG: 2048MB
    0x7FFFFFFF,                                     // KSEG0:  512MB
    0x1FFFFFFF,                                     // KSEG1:  512MB
    0xFFFFFFFF, 0xFFFFFFFF,                         // KSEG2: 1024MB
];

fn get_masked_address(address: u32) -> u32 {
    address & REGION_MASKS[(address >> 29) as usize]
} 