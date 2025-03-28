mod cop0;
mod instr;

use cop0::Cop0;
use instr::{Instruction, CPU_INSTRUCTIONS};

use super::bus::Bus;

use spdlog::prelude::*;

#[derive(Clone, Copy)]
struct DelaySlot {
    register: usize,
    value: u32
}

pub struct Cpu {
    regs: [u32; 32],
    program_counter: u32,
    program_counter_predictor: u32,

    delay_slots: [Option<DelaySlot>; 2],

    branch_delay: bool,
    branch_taken: bool,

    ex_program_counter: u32,
    ex_branch_delay: bool,
    ex_branch_taken: bool,

    cop0: Cop0,
    bus: Bus
}

impl Cpu {
    pub fn new(bus: Bus) -> Self {
        let mut regs = [0xDEADBEEF; 32];
        regs[0] = 0;
        
        Self {
            regs,
            program_counter: 0xBFC00000,
            program_counter_predictor: 0xBFC00004,
            branch_delay: false,
            branch_taken: false,

            ex_program_counter: 0xBFC00000,
            ex_branch_delay: false,
            ex_branch_taken: false,

            delay_slots: [None; 2],
            cop0: Cop0::new(),
            bus
        }
    }

    // NOTE: delay slots mechanics are temporarily copied from https://github.com/JaCzekanski/Avocado
    // It will be replaced in the future, perhaps I don't even need two slots

    fn set_reg(&mut self, register: usize, value: u32) {
        if register == 0 {
            return;
        }
        self.regs[register] = value;

        if let Some(slot) = self.delay_slots[0] {
            if slot.register == register {
                self.delay_slots[0] = None;
            }
        }
    }

    fn load_delay_slot(&mut self, register: usize, value: u32) {
        if register == 0 {
            return
        }

        if let Some(slot) = self.delay_slots[0] {
            if register == slot.register {
                self.delay_slots[0] = None;
            }
        }

        self.delay_slots[1] = Some(DelaySlot { register, value })
    }

    fn move_delay_slots(&mut self) {
        if let Some(slot0) = self.delay_slots[0] {
            self.regs[slot0.register] = slot0.value;
        }

        self.delay_slots[0] = self.delay_slots[1].take();
    }

    // TODO: Return ticks
    pub fn clock(&mut self) {
        let instr = self.fetch_instruction(self.program_counter);

        self.ex_program_counter = self.program_counter;

        self.program_counter = self.program_counter_predictor;
        self.program_counter_predictor = self.program_counter.wrapping_add(4);

        self.ex_branch_delay = self.branch_delay;
        self.ex_branch_taken = self.branch_taken;

        self.branch_delay = false;
        self.branch_taken = false;

        CPU_INSTRUCTIONS[instr.opcode()](self, instr);
        self.move_delay_slots();
    }

    fn fetch_instruction(&mut self, address: u32) -> Instruction {
        // TODO: Handle iCache
        Instruction(self.load32(address))
    }

    fn load32(&self, address: u32) -> u32 {
        self.bus.load32(address)
    }

    fn load16(&self, address: u32) -> u16 {
        self.bus.load16(address)
    }

    fn load8(&self, address: u32) -> u8 {
        self.bus.load8(address)
    }

    fn store32(&mut self, address: u32, value: u32) {
        if self.cop0.is_cache_isolated() {
            trace!("Cache is isolated, ignoring store32 for now");
            // TODO: Maintain iCache
            return;
        }
        self.bus.store32(address, value);
    }

    fn store16(&mut self, address: u32, value: u16) {
        if self.cop0.is_cache_isolated() {
            trace!("Cache is isolated, ignoring store16 for now");
            // TODO: Maintain iCache
            return;
        }
        self.bus.store16(address, value);
    }

    fn store8(&mut self, address: u32, value: u8) {
        if self.cop0.is_cache_isolated() {
            trace!("Cache is isolated, ignoring store8 for now");
            // TODO: Maintain iCache
            return;
        }
        self.bus.store8(address, value);
    }
}