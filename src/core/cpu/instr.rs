#![allow(dead_code, unused_variables)]

use super::Cpu;

use spdlog::prelude::*;

pub struct Instruction(pub u32);

impl Instruction {
    pub fn opcode(&self) -> usize {
        ((self.0 >> 26) & 0x3F) as usize
    }

    pub fn rs(&self) -> usize {
        ((self.0 >> 21) & 0x1F) as usize
    }

    pub fn rt(&self) -> usize {
        ((self.0 >> 16) & 0x1F) as usize
    }

    pub fn rd(&self) -> usize {
        ((self.0 >> 11) & 0x1F) as usize
    }

    pub fn shift(&self) -> usize {
        ((self.0 >> 6) & 0x1F) as usize
    }

    pub fn function(&self) -> usize {
        (self.0 & 0x3F) as usize
    }

    pub fn imm_zero(&self) -> u32 {
        self.0 & 0xFFFF
    }

    pub fn imm_signed(&self) -> u32 {
        (self.0 & 0xFFFF) as i16 as u32
    }

    pub fn target(&self) -> u32 {
        self.0 & 0x3FFFFFF
    }
}

// TODO: Maybe change lambdas to normal funcs

pub static CPU_INSTRUCTIONS: [fn(&mut Cpu); 0x40] = [
    |cpu| { CPU_SPECIAL_INSTRUCTIONS[0x00](cpu) },
    |cpu| { unimplemented!("BCONDZ") },
    |cpu| { unimplemented!("J") },
    |cpu| { unimplemented!("JAL") },
    |cpu| { unimplemented!("BEQ") },
    |cpu| { unimplemented!("BNE") },
    |cpu| { unimplemented!("BLEZ") },
    |cpu| { unimplemented!("BGTZ") },
    |cpu| { unimplemented!("ADDI") },
    |cpu| { unimplemented!("ADDIU") },
    |cpu| { unimplemented!("SLTI") },
    |cpu| { unimplemented!("SLTIU") },
    |cpu| { unimplemented!("ANDI") },
    |cpu| { unimplemented!("ORI") },
    |cpu| { unimplemented!("XORI") },
    |cpu| { unimplemented!("LUI") },
    |cpu| { unimplemented!("COP0") },
    |cpu| { unimplemented!("COP1") },
    |cpu| { unimplemented!("COP2") },
    |cpu| { unimplemented!("COP3") },
    op_illegal,
    op_illegal,
    op_illegal,
    op_illegal,
    op_illegal,
    op_illegal,
    op_illegal,
    op_illegal,
    op_illegal,
    op_illegal,
    op_illegal,
    op_illegal,
    |cpu| { unimplemented!("LB") },
    |cpu| { unimplemented!("LH") },
    |cpu| { unimplemented!("LWL") },
    |cpu| { unimplemented!("LW") },
    |cpu| { unimplemented!("LBU") },
    |cpu| { unimplemented!("LHU") },
    |cpu| { unimplemented!("LWR") },
    op_illegal,
    |cpu| { unimplemented!("SB") },
    |cpu| { unimplemented!("SH") },
    |cpu| { unimplemented!("SWL") },
    |cpu| { unimplemented!("SW") },
    op_illegal,
    op_illegal,
    |cpu| { unimplemented!("SWR") },
    op_illegal,
    |cpu| { unimplemented!("LWC0") },
    |cpu| { unimplemented!("LWC1") },
    |cpu| { unimplemented!("LWC2") },
    |cpu| { unimplemented!("LWC3") },
    op_illegal,
    op_illegal,
    op_illegal,
    op_illegal,
    |cpu| { unimplemented!("SWC0") },
    |cpu| { unimplemented!("SWC1") },
    |cpu| { unimplemented!("SWC2") },
    |cpu| { unimplemented!("SWC3") },
    op_illegal,
    op_illegal,
    op_illegal,
    op_illegal,
];

static CPU_SPECIAL_INSTRUCTIONS: [fn(&mut Cpu); 0x40] = [
    |cpu| { unimplemented!("SLL") },
    op_illegal,
    |cpu| { unimplemented!("SRL") },
    |cpu| { unimplemented!("SRA") },
    |cpu| { unimplemented!("SLLV") },
    op_illegal,
    |cpu| { unimplemented!("SRLV") },
    |cpu| { unimplemented!("SRAV") },
    |cpu| { unimplemented!("JR") },
    |cpu| { unimplemented!("JALR") },
    op_illegal,
    op_illegal,
    |cpu| { unimplemented!("SYSCALL") },
    |cpu| { unimplemented!("BREAK") },
    op_illegal,
    op_illegal,
    |cpu| { unimplemented!("MFHI") },
    |cpu| { unimplemented!("MTHI") },
    |cpu| { unimplemented!("MFLO") },
    |cpu| { unimplemented!("MTLO") },
    op_illegal,
    op_illegal,
    op_illegal,
    op_illegal,
    |cpu| { unimplemented!("MULT") },
    |cpu| { unimplemented!("MULTU") },
    |cpu| { unimplemented!("DIV") },
    |cpu| { unimplemented!("DIVU") },
    op_illegal,
    op_illegal,
    op_illegal,
    op_illegal,
    |cpu| { unimplemented!("ADD") },
    |cpu| { unimplemented!("ADDU") },
    |cpu| { unimplemented!("SUB") },
    |cpu| { unimplemented!("SUBU") },
    |cpu| { unimplemented!("AND") },
    |cpu| { unimplemented!("OR") },
    |cpu| { unimplemented!("XOR") },
    |cpu| { unimplemented!("NOR") },
    op_illegal,
    op_illegal,
    |cpu| { unimplemented!("SLT") },
    |cpu| { unimplemented!("SLTU") },
    op_illegal,
    op_illegal,
    op_illegal,
    op_illegal,
    op_illegal,
    op_illegal,
    op_illegal,
    op_illegal,
    op_illegal,
    op_illegal,
    op_illegal,
    op_illegal,
    op_illegal,
    op_illegal,
    op_illegal,
    op_illegal,
    op_illegal,
    op_illegal,
    op_illegal,
    op_illegal,
];

fn op_illegal(cpu: &mut Cpu) {
    critical!("Illegal instruction: 0x{:08X}", cpu.instruction.0);
    std::process::exit(1);
}