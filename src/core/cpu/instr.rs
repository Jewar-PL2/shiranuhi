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
    |cpu, instr| { jump(cpu, (cpu.program_counter & 0xF0000000) | (instr.target() << 2)); },
    |cpu, instr| {
        info!("[CPU] Linking return address: 0x{:08X}", cpu.program_counter_predictor);
        cpu.set_reg(31, cpu.program_counter_predictor);

        jump(cpu, (cpu.program_counter & 0xF0000000) | (instr.target() << 2));
    },
    |cpu, instr| { 
        cpu.branch_delay = true;
        if cpu.regs[instr.rs()] == cpu.regs[instr.rt()] {
            branch(cpu, instr);
        } 
    },
    |cpu, instr| { 
        cpu.branch_delay = true;
        if cpu.regs[instr.rs()] != cpu.regs[instr.rt()] {
            branch(cpu, instr);
        }
    },
    |cpu, instr| { unimplemented!("BLEZ") },
    |cpu, instr| { unimplemented!("BGTZ") },
    |cpu, instr| { 
        trace!("[ADDI] rs: 0x{:08X}, imm: 0x{:08X}", cpu.regs[instr.rs()], instr.imm_signed());

        let rs = cpu.regs[instr.rs()];
        match rs.checked_add(instr.imm_signed()) {
            Some(value) => cpu.set_reg(instr.rt(), value),
            None => todo!("Exception")
        }
    },
    |cpu, instr| { cpu.set_reg(instr.rt(), cpu.regs[instr.rs()].wrapping_add(instr.imm_signed())); },
    |cpu, instr| { unimplemented!("SLTI") },
    |cpu, instr| { unimplemented!("SLTIU") },
    |cpu, instr| { cpu.set_reg(instr.rt(), cpu.regs[instr.rs()] & instr.imm_zero()); },
    |cpu, instr| { cpu.set_reg(instr.rt(), cpu.regs[instr.rs()] | instr.imm_zero()); },
    |cpu, instr| { unimplemented!("XORI") },
    |cpu, instr| { cpu.set_reg(instr.rt(), instr.imm_zero() << 16); },
    |cpu, instr| { 
        match instr.rs() {
            0 => {
                let value = cpu.cop0.load(instr.rd());
                match value {
                    Some(value) => cpu.load_delay_slot(instr.rt(), value),
                    None => todo!("Exception")
                }
            }
            4 => cpu.cop0.store(instr.rd(), cpu.regs[instr.rt()]),
            16 => unimplemented!("RFE"),
            _ => unreachable!()
        }
    },
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
    |cpu, instr| {
        let address = cpu.regs[instr.rs()].wrapping_add(instr.imm_signed());
        let value = cpu.load8(address) as i8;

        cpu.load_delay_slot(instr.rt(), value as u32);
    },
    |cpu, instr| { unimplemented!("LH") },
    |cpu, instr| { unimplemented!("LWL") },
    |cpu, instr| { 
        let address = cpu.regs[instr.rs()].wrapping_add(instr.imm_signed());
        // TODO: Handle misalignments
        let value = cpu.load32(address);

        cpu.load_delay_slot(instr.rt(), value);
    },
    |cpu, instr| { unimplemented!("LBU") },
    |cpu, instr| { unimplemented!("LHU") },
    |cpu, instr| { unimplemented!("LWR") },
    op_illegal,
    |cpu, instr| {
        let address = cpu.regs[instr.rs()].wrapping_add(instr.imm_signed());
        cpu.store8(address, cpu.regs[instr.rt()] as u8);
    },
    |cpu, instr| { 
        let address = cpu.regs[instr.rs()].wrapping_add(instr.imm_signed());
        // TODO: Handle misalignments

        cpu.store16(address, cpu.regs[instr.rt()] as u16);
    },
    |cpu, instr| { unimplemented!("SWL") },
    |cpu, instr| { 
        let address = cpu.regs[instr.rs()].wrapping_add(instr.imm_signed());
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
    |cpu, instr| { jump(cpu, cpu.regs[instr.rs()]); },
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
    |cpu, instr| { cpu.set_reg(instr.rd(), cpu.regs[instr.rs()].wrapping_add(cpu.regs[instr.rt()])); },
    |cpu, instr| { unimplemented!("SUB") },
    |cpu, instr| { unimplemented!("SUBU") },
    |cpu, instr| { cpu.set_reg(instr.rd(), cpu.regs[instr.rs()] & cpu.regs[instr.rt()]); },
    |cpu, instr| { cpu.set_reg(instr.rd(), cpu.regs[instr.rs()] | cpu.regs[instr.rt()]); },
    |cpu, instr| { unimplemented!("XOR") },
    |cpu, instr| { unimplemented!("NOR") },
    op_illegal,
    op_illegal,
    |cpu, instr| { unimplemented!("SLT") },
    |cpu, instr| { cpu.set_reg(instr.rd(), if cpu.regs[instr.rs()] < cpu.regs[instr.rt()] { 1 } else { 0 }) },
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

fn jump(cpu: &mut Cpu, address: u32) {
    cpu.branch_delay = true;
    cpu.branch_taken = true;

    let jump_address = address;
    info!("[CPU] Performing jump to address: 0x{:08X}", jump_address);
    cpu.program_counter_predictor = jump_address;
}

fn branch(cpu: &mut Cpu, instr: Instruction) {
    // TODO: Verify if its correct
    let branch_address = cpu.program_counter.wrapping_add(instr.imm_signed() << 2);
    trace!("[CPU] Performing branch to address: 0x{:08X}", branch_address);
    
    cpu.branch_taken = true;
    cpu.program_counter_predictor = branch_address;
}