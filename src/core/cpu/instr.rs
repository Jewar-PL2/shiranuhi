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

pub static CPU_INSTRUCTIONS: [fn(&mut Cpu, Instruction); 0x40] = [
    |cpu, instr| { CPU_SPECIAL_INSTRUCTIONS[instr.function()](cpu, instr) },
    |cpu, instr| { unimplemented!("BCONDZ") },
    |cpu, instr| { 
        // TODO: Set flags
        
        let jump_address = (cpu.program_counter & 0xF0000000) | (instr.target() * 4);
        trace!("CPU: Performing jump to address: 0x{:08X}", jump_address);
        cpu.program_counter_predictor = jump_address;
    },
    |cpu, instr| { unimplemented!("JAL") },
    |cpu, instr| { unimplemented!("BEQ") },
    |cpu, instr| { unimplemented!("BNE") },
    |cpu, instr| { unimplemented!("BLEZ") },
    |cpu, instr| { unimplemented!("BGTZ") },
    |cpu, instr| { unimplemented!("ADDI") },
    |cpu, instr| { cpu.set_reg(instr.rt(), cpu.regs[instr.rs()] + instr.imm_signed()); },
    |cpu, instr| { unimplemented!("SLTI") },
    |cpu, instr| { unimplemented!("SLTIU") },
    |cpu, instr| { unimplemented!("ANDI") },
    |cpu, instr| { cpu.set_reg(instr.rt(), cpu.regs[instr.rs()] | instr.imm_zero()); },
    |cpu, instr| { unimplemented!("XORI") },
    |cpu, instr| { cpu.set_reg(instr.rt(), instr.imm_zero() << 16); },
    |cpu, instr| { unimplemented!("COP0") },
    |cpu, instr| { unimplemented!("COP1") },
    |cpu, instr| { unimplemented!("COP2") },
    |cpu, instr| { unimplemented!("COP3") },
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
    |cpu, instr| { unimplemented!("LB") },
    |cpu, instr| { unimplemented!("LH") },
    |cpu, instr| { unimplemented!("LWL") },
    |cpu, instr| { unimplemented!("LW") },
    |cpu, instr| { unimplemented!("LBU") },
    |cpu, instr| { unimplemented!("LHU") },
    |cpu, instr| { unimplemented!("LWR") },
    op_illegal,
    |cpu, instr| { unimplemented!("SB") },
    |cpu, instr| { unimplemented!("SH") },
    |cpu, instr| { unimplemented!("SWL") },
    |cpu, instr| { 
        let address = cpu.regs[instr.rs()] + instr.imm_signed();
        // TODO: Handle misalignments

        cpu.store32(address, cpu.regs[instr.rt()]);
    },
    op_illegal,
    op_illegal,
    |cpu, instr| { unimplemented!("SWR") },
    op_illegal,
    |cpu, instr| { unimplemented!("LWC0") },
    |cpu, instr| { unimplemented!("LWC1") },
    |cpu, instr| { unimplemented!("LWC2") },
    |cpu, instr| { unimplemented!("LWC3") },
    op_illegal,
    op_illegal,
    op_illegal,
    op_illegal,
    |cpu, instr| { unimplemented!("SWC0") },
    |cpu, instr| { unimplemented!("SWC1") },
    |cpu, instr| { unimplemented!("SWC2") },
    |cpu, instr| { unimplemented!("SWC3") },
    op_illegal,
    op_illegal,
    op_illegal,
    op_illegal,
];

static CPU_SPECIAL_INSTRUCTIONS: [fn(&mut Cpu, Instruction); 0x40] = [
    |cpu, instr| {
        if instr.0 == 0 {
            return
        }
        cpu.set_reg(instr.rd(), cpu.regs[instr.rt()] << instr.shift());
    },
    op_illegal,
    |cpu, instr| { unimplemented!("SRL") },
    |cpu, instr| { unimplemented!("SRA") },
    |cpu, instr| { unimplemented!("SLLV") },
    op_illegal,
    |cpu, instr| { unimplemented!("SRLV") },
    |cpu, instr| { unimplemented!("SRAV") },
    |cpu, instr| { unimplemented!("JR") },
    |cpu, instr| { unimplemented!("JALR") },
    op_illegal,
    op_illegal,
    |cpu, instr| { unimplemented!("SYSCALL") },
    |cpu, instr| { unimplemented!("BREAK") },
    op_illegal,
    op_illegal,
    |cpu, instr| { unimplemented!("MFHI") },
    |cpu, instr| { unimplemented!("MTHI") },
    |cpu, instr| { unimplemented!("MFLO") },
    |cpu, instr| { unimplemented!("MTLO") },
    op_illegal,
    op_illegal,
    op_illegal,
    op_illegal,
    |cpu, instr| { unimplemented!("MULT") },
    |cpu, instr| { unimplemented!("MULTU") },
    |cpu, instr| { unimplemented!("DIV") },
    |cpu, instr| { unimplemented!("DIVU") },
    op_illegal,
    op_illegal,
    op_illegal,
    op_illegal,
    |cpu, instr| { unimplemented!("ADD") },
    |cpu, instr| { unimplemented!("ADDU") },
    |cpu, instr| { unimplemented!("SUB") },
    |cpu, instr| { unimplemented!("SUBU") },
    |cpu, instr| { unimplemented!("AND") },
    |cpu, instr| { unimplemented!("OR") },
    |cpu, instr| { unimplemented!("XOR") },
    |cpu, instr| { unimplemented!("NOR") },
    op_illegal,
    op_illegal,
    |cpu, instr| { unimplemented!("SLT") },
    |cpu, instr| { unimplemented!("SLTU") },
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

fn op_illegal(cpu: &mut Cpu, instr: Instruction) {
    critical!("Illegal instruction: 0x{:08X}", instr.0);
    std::process::exit(1);
}