mod cop0;
mod instr;

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
            delay_slots: [None; 2],
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

        self.program_counter = self.program_counter_predictor;
        self.program_counter_predictor = self.program_counter.wrapping_add(4);

        CPU_INSTRUCTIONS[instr.opcode()](self, instr);
    }

    fn fetch_instruction(&mut self, address: u32) -> Instruction {
        // TODO: Handle iCache
        Instruction(self.load32(address))
    }

    fn load32(&self, address: u32) -> u32 {
        self.bus.load32(address)
    }

    fn store32(&mut self, address: u32, value: u32) {
        // TODO: Handle cache isolation
        self.bus.store32(address, value);
    }
}