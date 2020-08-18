use super::bus::Bus;
use super::cpu::{AddressingMode, Instruction, Opcode, INSTRUCTION_LOOKUP};
use std::fmt;
use std::io::Write;

fn get_instruction_size(ins: &Instruction) -> u16 {
    match ins.mode {
        AddressingMode::ABS => 3,
        AddressingMode::ABX => 3,
        AddressingMode::ABY => 3,
        AddressingMode::IMM => 2,
        AddressingMode::REL => 2,
        AddressingMode::ACC => 1,
        AddressingMode::IMP => 1,
        AddressingMode::IND => 3,
        AddressingMode::ZP0 => 2,
        AddressingMode::ZPX => 2,
        AddressingMode::ZPY => 2,
        AddressingMode::IZX => 2,
        AddressingMode::IZY => 2,
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Opcode::ADC => "ADC",
            Opcode::AND => "AND",
            Opcode::ASL => "ASL",
            Opcode::BCC => "BCC",
            Opcode::BCS => "BCS",
            Opcode::BEQ => "BEQ",
            Opcode::BIT => "BIT",
            Opcode::BMI => "BMI",
            Opcode::BNE => "BNE",
            Opcode::BPL => "BPL",
            Opcode::BRK => "BRK",
            Opcode::BVC => "BVC",
            Opcode::BVS => "BVS",
            Opcode::CLC => "CLC",
            Opcode::CLD => "CLD",
            Opcode::CLI => "CLI",
            Opcode::CLV => "CLV",
            Opcode::CMP => "CMP",
            Opcode::CPX => "CPX",
            Opcode::CPY => "CPY",
            Opcode::DEC => "DEC",
            Opcode::DEX => "DEX",
            Opcode::DEY => "DEY",
            Opcode::EOR => "EOR",
            Opcode::ERR => "???",
            Opcode::INC => "INC",
            Opcode::INX => "INX",
            Opcode::INY => "INY",
            Opcode::JMP => "JMP",
            Opcode::JSR => "JSR",
            Opcode::LDA => "LDA",
            Opcode::LDX => "LDX",
            Opcode::LDY => "LDY",
            Opcode::LSR => "LSR",
            Opcode::NOP => "NOP",
            Opcode::ORA => "ORA",
            Opcode::PHA => "PHA",
            Opcode::PHP => "PHP",
            Opcode::PLA => "PLA",
            Opcode::PLP => "PLP",
            Opcode::ROL => "ROL",
            Opcode::ROR => "ROR",
            Opcode::RTI => "RTI",
            Opcode::RTS => "RTS",
            Opcode::SBC => "SBC",
            Opcode::SEC => "SEC",
            Opcode::SED => "SED",
            Opcode::SEI => "SEI",
            Opcode::STA => "STA",
            Opcode::STX => "STX",
            Opcode::STY => "STY",
            Opcode::TAX => "TAX",
            Opcode::TAY => "TAY",
            Opcode::TSX => "TSX",
            Opcode::TXA => "TXA",
            Opcode::TXS => "TXS",
            Opcode::TYA => "TYA",
        };
        write!(f, "{}", name)
    }
}

#[allow(dead_code)]
pub fn disasm(out: &mut impl Write, bus: &mut Bus, addr: u16) {
    let ins_code = bus.read(addr);

    let ins = &INSTRUCTION_LOOKUP[ins_code as usize];

    let ins_size = get_instruction_size(ins);
    for i in 0..4 {
        if i < ins_size {
            write!(out, "{:02X} ", bus.read(addr + i)).unwrap();
        } else {
            write!(out, "   ").unwrap();
        }
    }

    write!(out, "{} ", ins.opcode).unwrap();
}
