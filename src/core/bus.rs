use super::{
    bios::Bios,
    devices::ram::Ram
};
use spdlog::prelude::*;

const RAM_RANGE: Range = Range(0x00000000, 2 * 1024 * 1024);
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

        if CACHE_CONTROL_RANGE.contains(address).is_some() {
            return self.cache_control.0;
        }

        panic!("INVALID ADDRESS: 0x{:08X}", address);
    }

    pub fn store32(&mut self, address: u32, value: u32) {
        let address = get_masked_address(address);

        if let Some(offset) = RAM_RANGE.contains(address) {
            self.ram.store32(offset, value);
            return;
        }

        if CACHE_CONTROL_RANGE.contains(address).is_some() {
            self.cache_control.0 = value;
            return;
        }

        warn!("Unhandled store32 at [0x{:08X}]: 0x{:08X}", address, value)
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