use super::bus::Bus;

#[derive(Default, Clone)]
#[allow(non_snake_case)]
pub struct Flags {
    pub C: bool, // Carry Flag
    pub Z: bool, // Zero Flag
    pub I: bool, // Interrupt Disable
    pub D: bool, // Decimal Mode
    pub B: bool, // Break Command
    pub U: bool, // Unused
    pub O: bool, // Overflow Flag
    pub N: bool, // Negative Flags
}
#[allow(non_snake_case)]
pub struct Cpu {
    pub bus: Bus,
    pub PC: u16,
    pub SP: u8, // ?
    pub A: u8,
    pub X: u8,
    pub Y: u8,
    pub flags: Flags,
    pub total_cycles: usize,
    pub cycles: u8,
}

pub fn to_u16(hi: u8, lo: u8) -> u16 {
    return (hi as u16) << 8 | (lo as u16);
}

#[derive(Debug, PartialEq)]
pub enum AddressingMode {
    ABS, // Absolute
    ABX, //
    ABY, //
    ACC, // Accumulator
    IMM, // Immediate
    IMP, //
    IND, // Indirect
    IZX, //
    IZY, //
    REL, // Relative
    ZP0, // Zero Page 0
    ZPX, // Zero Page X
    ZPY, // Zero Page Y
}

#[derive(Debug, PartialEq)]
pub enum Opcode {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    ERR, // Unknown opcode -> Error
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
}

pub struct Instruction {
    pub opcode: Opcode,
    pub mode: AddressingMode,
    pub cycles: u8,
}

#[rustfmt::skip]
pub const INSTRUCTION_LOOKUP: [Instruction; 256] = [
		// 0x00
        Instruction { opcode: Opcode::BRK, mode: AddressingMode::IMP, cycles: 7 },
        Instruction { opcode: Opcode::ORA, mode: AddressingMode::IZX, cycles: 6 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 8 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 3 },
        Instruction { opcode: Opcode::ORA, mode: AddressingMode::ZP0, cycles: 3 },
        Instruction { opcode: Opcode::ASL, mode: AddressingMode::ZP0, cycles: 5 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 5 },
        Instruction { opcode: Opcode::PHP, mode: AddressingMode::IMP, cycles: 3 },
        Instruction { opcode: Opcode::ORA, mode: AddressingMode::IMM, cycles: 2 },
        Instruction { opcode: Opcode::ASL, mode: AddressingMode::ACC, cycles: 2 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 4 },
        Instruction { opcode: Opcode::ORA, mode: AddressingMode::ABS, cycles: 4 },
        Instruction { opcode: Opcode::ASL, mode: AddressingMode::ABS, cycles: 6 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 6 },
        // 0x10
		Instruction { opcode: Opcode::BPL, mode: AddressingMode::REL, cycles: 2 },
        Instruction { opcode: Opcode::ORA, mode: AddressingMode::IZY, cycles: 5 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 8 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 4 },
        Instruction { opcode: Opcode::ORA, mode: AddressingMode::ZPX, cycles: 4 },
        Instruction { opcode: Opcode::ASL, mode: AddressingMode::ZPX, cycles: 6 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 6 },
        Instruction { opcode: Opcode::CLC, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::ORA, mode: AddressingMode::ABY, cycles: 4 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 7 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 4 },
        Instruction { opcode: Opcode::ORA, mode: AddressingMode::ABX, cycles: 4 },
        Instruction { opcode: Opcode::ASL, mode: AddressingMode::ABX, cycles: 7 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 7 },
        // 0x20
		Instruction { opcode: Opcode::JSR, mode: AddressingMode::ABS, cycles: 6 },
        Instruction { opcode: Opcode::AND, mode: AddressingMode::IZX, cycles: 6 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 8 },
        Instruction { opcode: Opcode::BIT, mode: AddressingMode::ZP0, cycles: 3 },
        Instruction { opcode: Opcode::AND, mode: AddressingMode::ZP0, cycles: 3 },
        Instruction { opcode: Opcode::ROL, mode: AddressingMode::ZP0, cycles: 5 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 5 },
        Instruction { opcode: Opcode::PLP, mode: AddressingMode::IMP, cycles: 4 },
        Instruction { opcode: Opcode::AND, mode: AddressingMode::IMM, cycles: 2 },
        Instruction { opcode: Opcode::ROL, mode: AddressingMode::ACC, cycles: 2 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::BIT, mode: AddressingMode::ABS, cycles: 4 },
        Instruction { opcode: Opcode::AND, mode: AddressingMode::ABS, cycles: 4 },
        Instruction { opcode: Opcode::ROL, mode: AddressingMode::ABS, cycles: 6 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 6 },
        // 0x30
		Instruction { opcode: Opcode::BMI, mode: AddressingMode::REL, cycles: 2 },
        Instruction { opcode: Opcode::AND, mode: AddressingMode::IZY, cycles: 5 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 8 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 4 },
        Instruction { opcode: Opcode::AND, mode: AddressingMode::ZPX, cycles: 4 },
        Instruction { opcode: Opcode::ROL, mode: AddressingMode::ZPX, cycles: 6 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 6 },
        Instruction { opcode: Opcode::SEC, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::AND, mode: AddressingMode::ABY, cycles: 4 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 7 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 4 },
        Instruction { opcode: Opcode::AND, mode: AddressingMode::ABX, cycles: 4 },
        Instruction { opcode: Opcode::ROL, mode: AddressingMode::ABX, cycles: 7 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 7 },
        // 0x40
		Instruction { opcode: Opcode::RTI, mode: AddressingMode::IMP, cycles: 6 },
        Instruction { opcode: Opcode::EOR, mode: AddressingMode::IZX, cycles: 6 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 8 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 3 },
        Instruction { opcode: Opcode::EOR, mode: AddressingMode::ZP0, cycles: 3 },
        Instruction { opcode: Opcode::LSR, mode: AddressingMode::ZP0, cycles: 5 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 5 },
        Instruction { opcode: Opcode::PHA, mode: AddressingMode::IMP, cycles: 3 },
        Instruction { opcode: Opcode::EOR, mode: AddressingMode::IMM, cycles: 2 },
        Instruction { opcode: Opcode::LSR, mode: AddressingMode::ACC, cycles: 2 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::JMP, mode: AddressingMode::ABS, cycles: 3 },
        Instruction { opcode: Opcode::EOR, mode: AddressingMode::ABS, cycles: 4 },
        Instruction { opcode: Opcode::LSR, mode: AddressingMode::ABS, cycles: 6 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 6 },
        // 0x50
		Instruction { opcode: Opcode::BVC, mode: AddressingMode::REL, cycles: 2 },
        Instruction { opcode: Opcode::EOR, mode: AddressingMode::IZY, cycles: 5 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 8 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 4 },
        Instruction { opcode: Opcode::EOR, mode: AddressingMode::ZPX, cycles: 4 },
        Instruction { opcode: Opcode::LSR, mode: AddressingMode::ZPX, cycles: 6 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 6 },
        Instruction { opcode: Opcode::CLI, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::EOR, mode: AddressingMode::ABY, cycles: 4 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 7 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 4 },
        Instruction { opcode: Opcode::EOR, mode: AddressingMode::ABX, cycles: 4 },
        Instruction { opcode: Opcode::LSR, mode: AddressingMode::ABX, cycles: 7 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 7 },
        // 0x60
		Instruction { opcode: Opcode::RTS, mode: AddressingMode::IMP, cycles: 6 },
        Instruction { opcode: Opcode::ADC, mode: AddressingMode::IZX, cycles: 6 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 8 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 3 },
        Instruction { opcode: Opcode::ADC, mode: AddressingMode::ZP0, cycles: 3 },
        Instruction { opcode: Opcode::ROR, mode: AddressingMode::ZP0, cycles: 5 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 5 },
        Instruction { opcode: Opcode::PLA, mode: AddressingMode::IMP, cycles: 4 },
        Instruction { opcode: Opcode::ADC, mode: AddressingMode::IMM, cycles: 2 },
        Instruction { opcode: Opcode::ROR, mode: AddressingMode::ACC, cycles: 2 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::JMP, mode: AddressingMode::IND, cycles: 5 },
        Instruction { opcode: Opcode::ADC, mode: AddressingMode::ABS, cycles: 4 },
        Instruction { opcode: Opcode::ROR, mode: AddressingMode::ABS, cycles: 6 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 6 },
        // 0x70
		Instruction { opcode: Opcode::BVS, mode: AddressingMode::REL, cycles: 2 },
        Instruction { opcode: Opcode::ADC, mode: AddressingMode::IZY, cycles: 5 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 8 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 4 },
        Instruction { opcode: Opcode::ADC, mode: AddressingMode::ZPX, cycles: 4 },
        Instruction { opcode: Opcode::ROR, mode: AddressingMode::ZPX, cycles: 6 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 6 },
        Instruction { opcode: Opcode::SEI, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::ADC, mode: AddressingMode::ABY, cycles: 4 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 7 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 4 },
        Instruction { opcode: Opcode::ADC, mode: AddressingMode::ABX, cycles: 4 },
        Instruction { opcode: Opcode::ROR, mode: AddressingMode::ABX, cycles: 7 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 7 },
        // 0x80
		Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::STA, mode: AddressingMode::IZX, cycles: 6 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 6 },
        Instruction { opcode: Opcode::STY, mode: AddressingMode::ZP0, cycles: 3 },
        Instruction { opcode: Opcode::STA, mode: AddressingMode::ZP0, cycles: 3 },
        Instruction { opcode: Opcode::STX, mode: AddressingMode::ZP0, cycles: 3 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 3 },
        Instruction { opcode: Opcode::DEY, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::TXA, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::STY, mode: AddressingMode::ABS, cycles: 4 },
        Instruction { opcode: Opcode::STA, mode: AddressingMode::ABS, cycles: 4 },
        Instruction { opcode: Opcode::STX, mode: AddressingMode::ABS, cycles: 4 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 4 },
        // 0x90
		Instruction { opcode: Opcode::BCC, mode: AddressingMode::REL, cycles: 2 },
        Instruction { opcode: Opcode::STA, mode: AddressingMode::IZY, cycles: 6 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 6 },
        Instruction { opcode: Opcode::STY, mode: AddressingMode::ZPX, cycles: 4 },
        Instruction { opcode: Opcode::STA, mode: AddressingMode::ZPX, cycles: 4 },
        Instruction { opcode: Opcode::STX, mode: AddressingMode::ZPY, cycles: 4 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 4 },
        Instruction { opcode: Opcode::TYA, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::STA, mode: AddressingMode::ABY, cycles: 5 },
        Instruction { opcode: Opcode::TXS, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 5 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 5 },
        Instruction { opcode: Opcode::STA, mode: AddressingMode::ABX, cycles: 5 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 5 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 5 },
        // 0xA0
		Instruction { opcode: Opcode::LDY, mode: AddressingMode::IMM, cycles: 2 },
        Instruction { opcode: Opcode::LDA, mode: AddressingMode::IZX, cycles: 6 },
        Instruction { opcode: Opcode::LDX, mode: AddressingMode::IMM, cycles: 2 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 6 },
        Instruction { opcode: Opcode::LDY, mode: AddressingMode::ZP0, cycles: 3 },
        Instruction { opcode: Opcode::LDA, mode: AddressingMode::ZP0, cycles: 3 },
        Instruction { opcode: Opcode::LDX, mode: AddressingMode::ZP0, cycles: 3 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 3 },
        Instruction { opcode: Opcode::TAY, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::LDA, mode: AddressingMode::IMM, cycles: 2 },
        Instruction { opcode: Opcode::TAX, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::LDY, mode: AddressingMode::ABS, cycles: 4 },
        Instruction { opcode: Opcode::LDA, mode: AddressingMode::ABS, cycles: 4 },
        Instruction { opcode: Opcode::LDX, mode: AddressingMode::ABS, cycles: 4 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 4 },
        // 0xB0
		Instruction { opcode: Opcode::BCS, mode: AddressingMode::REL, cycles: 2 },
        Instruction { opcode: Opcode::LDA, mode: AddressingMode::IZY, cycles: 5 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 5 },
        Instruction { opcode: Opcode::LDY, mode: AddressingMode::ZPX, cycles: 4 },
        Instruction { opcode: Opcode::LDA, mode: AddressingMode::ZPX, cycles: 4 },
        Instruction { opcode: Opcode::LDX, mode: AddressingMode::ZPY, cycles: 4 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 4 },
        Instruction { opcode: Opcode::CLV, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::LDA, mode: AddressingMode::ABY, cycles: 4 },
        Instruction { opcode: Opcode::TSX, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 4 },
        Instruction { opcode: Opcode::LDY, mode: AddressingMode::ABX, cycles: 4 },
        Instruction { opcode: Opcode::LDA, mode: AddressingMode::ABX, cycles: 4 },
        Instruction { opcode: Opcode::LDX, mode: AddressingMode::ABY, cycles: 4 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 4 },
        // 0xC0
		Instruction { opcode: Opcode::CPY, mode: AddressingMode::IMM, cycles: 2 },
        Instruction { opcode: Opcode::CMP, mode: AddressingMode::IZX, cycles: 6 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 8 },
        Instruction { opcode: Opcode::CPY, mode: AddressingMode::ZP0, cycles: 3 },
        Instruction { opcode: Opcode::CMP, mode: AddressingMode::ZP0, cycles: 3 },
        Instruction { opcode: Opcode::DEC, mode: AddressingMode::ZP0, cycles: 5 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 5 },
        Instruction { opcode: Opcode::INY, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::CMP, mode: AddressingMode::IMM, cycles: 2 },
        Instruction { opcode: Opcode::DEX, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::CPY, mode: AddressingMode::ABS, cycles: 4 },
        Instruction { opcode: Opcode::CMP, mode: AddressingMode::ABS, cycles: 4 },
        Instruction { opcode: Opcode::DEC, mode: AddressingMode::ABS, cycles: 6 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 6 },
        // 0xD0
		Instruction { opcode: Opcode::BNE, mode: AddressingMode::REL, cycles: 2 },
        Instruction { opcode: Opcode::CMP, mode: AddressingMode::IZY, cycles: 5 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 8 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 4 },
        Instruction { opcode: Opcode::CMP, mode: AddressingMode::ZPX, cycles: 4 },
        Instruction { opcode: Opcode::DEC, mode: AddressingMode::ZPX, cycles: 6 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 6 },
        Instruction { opcode: Opcode::CLD, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::CMP, mode: AddressingMode::ABY, cycles: 4 },
        Instruction { opcode: Opcode::NOP, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 7 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 4 },
        Instruction { opcode: Opcode::CMP, mode: AddressingMode::ABX, cycles: 4 },
        Instruction { opcode: Opcode::DEC, mode: AddressingMode::ABX, cycles: 7 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 7 },
        // 0xE0
		Instruction { opcode: Opcode::CPX, mode: AddressingMode::IMM, cycles: 2 },
        Instruction { opcode: Opcode::SBC, mode: AddressingMode::IZX, cycles: 6 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 8 },
        Instruction { opcode: Opcode::CPX, mode: AddressingMode::ZP0, cycles: 3 },
        Instruction { opcode: Opcode::SBC, mode: AddressingMode::ZP0, cycles: 3 },
        Instruction { opcode: Opcode::INC, mode: AddressingMode::ZP0, cycles: 5 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 5 },
        Instruction { opcode: Opcode::INX, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::SBC, mode: AddressingMode::IMM, cycles: 2 },
        Instruction { opcode: Opcode::NOP, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::CPX, mode: AddressingMode::ABS, cycles: 4 },
        Instruction { opcode: Opcode::SBC, mode: AddressingMode::ABS, cycles: 4 },
        Instruction { opcode: Opcode::INC, mode: AddressingMode::ABS, cycles: 6 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 6 },
        // 0xF0
		Instruction { opcode: Opcode::BEQ, mode: AddressingMode::REL, cycles: 2 },
        Instruction { opcode: Opcode::SBC, mode: AddressingMode::IZY, cycles: 5 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 8 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 4 },
        Instruction { opcode: Opcode::SBC, mode: AddressingMode::ZPX, cycles: 4 },
        Instruction { opcode: Opcode::INC, mode: AddressingMode::ZPX, cycles: 6 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 6 },
        Instruction { opcode: Opcode::SED, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::SBC, mode: AddressingMode::ABY, cycles: 4 },
        Instruction { opcode: Opcode::NOP, mode: AddressingMode::IMP, cycles: 2 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 7 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 4 },
        Instruction { opcode: Opcode::SBC, mode: AddressingMode::ABX, cycles: 4 },
        Instruction { opcode: Opcode::INC, mode: AddressingMode::ABX, cycles: 7 },
        Instruction { opcode: Opcode::ERR, mode: AddressingMode::IMP, cycles: 7 },
];

impl Flags {
    pub fn set_zn(&mut self, v: u8) {
        self.Z = v == 0;
        self.N = v & 0x80 != 0;
    }

    pub fn to_byte(&self) -> u8 {
        (self.C as u8) << 0
            | (self.Z as u8) << 1
            | (self.I as u8) << 2
            | (self.D as u8) << 3
            | (self.B as u8) << 4
            | (self.U as u8) << 5
            | (self.O as u8) << 6
            | (self.N as u8) << 7
    }

    pub fn set_byte(&mut self, b: u8) {
        self.C = ((b >> 0) & 1) != 0;
        self.Z = ((b >> 1) & 1) != 0;
        self.I = ((b >> 2) & 1) != 0;
        self.D = ((b >> 3) & 1) != 0;
        self.O = ((b >> 6) & 1) != 0;
        self.N = ((b >> 7) & 1) != 0;
    }
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            bus: Bus::new(),
            PC: 0,
            SP: 0,
            A: 0,
            X: 0,
            Y: 0,
            flags: Flags::default(),
            total_cycles: 0,
            cycles: 0,
        }
    }

    pub fn reset(&mut self) {
        self.PC = self.read_from_location_u16(0xFFFC);
        self.SP = 0xFD;
        self.total_cycles = 0;
        self.cycles = 0;
        self.A = 0;
        self.X = 0;
        self.Y = 0;
        self.flags = Flags::default();
        self.flags.I = true;
        self.flags.U = true;
        self.cycles = 7;
    }

    pub fn nmi(&mut self) {
        self.push_u16(self.PC);

        self.flags.B = false;
        self.flags.U = true;
        self.flags.I = true;

        self.push_flags();

        self.PC = self.read_from_location_u16(0xFFFA);

        self.cycles = 8;
    }

    pub fn clock(&mut self) {
        self.total_cycles += 1;

        if self.cycles > 0 {
            self.cycles -= 1;
            return;
        }

        let ins_code = self.bus.cpu_read(self.PC);
        self.PC += 1;

        let ins = &INSTRUCTION_LOOKUP[ins_code as usize];

        self.cycles = ins.cycles;
        let addr = self.read_addr(&ins);

        match ins.opcode {
            Opcode::JMP => {
                self.PC = addr;
            }
            Opcode::LDX => {
                let v = self.bus.cpu_read(addr);
                self.X = v as u8;
                self.flags.set_zn(self.X);
            }
            Opcode::LDA => {
                let v = self.bus.cpu_read(addr);
                self.A = v as u8;
                self.flags.set_zn(self.A);
            }
            Opcode::LDY => {
                let v = self.bus.cpu_read(addr);
                self.Y = v as u8;
                self.flags.set_zn(self.Y);
            }
            Opcode::JSR => {
                self.PC -= 1;
                self.push_u16(self.PC);
                self.PC = addr;
            }
            Opcode::RTS => {
                self.PC = self.pop_u16();
                self.PC += 1;
            }
            Opcode::NOP => {}
            Opcode::SEC => {
                self.flags.C = true;
            }
            Opcode::CLC => {
                self.flags.C = false;
            }
            Opcode::SED => {
                self.flags.D = true;
            }
            Opcode::CLD => {
                self.flags.D = false;
            }
            Opcode::SEI => {
                self.flags.I = true;
            }
            Opcode::CLI => {
                self.flags.I = false;
            }
            Opcode::CLV => {
                self.flags.O = false;
            }
            Opcode::BCS => {
                if self.flags.C {
                    self.branch_jump(addr);
                }
            }
            Opcode::BCC => {
                if !self.flags.C {
                    self.branch_jump(addr);
                }
            }
            Opcode::BEQ => {
                if self.flags.Z {
                    self.branch_jump(addr);
                }
            }
            Opcode::BNE => {
                if !self.flags.Z {
                    self.branch_jump(addr);
                }
            }
            Opcode::BMI => {
                if self.flags.N {
                    self.branch_jump(addr);
                }
            }
            Opcode::BPL => {
                if !self.flags.N {
                    self.branch_jump(addr);
                }
            }
            Opcode::BVC => {
                if !self.flags.O {
                    self.branch_jump(addr);
                }
            }
            Opcode::BVS => {
                if self.flags.O {
                    self.branch_jump(addr);
                }
            }
            Opcode::STA => {
                self.bus.cpu_write(addr, self.A);
            }
            Opcode::STX => {
                self.bus.cpu_write(addr, self.X);
            }
            Opcode::STY => {
                self.bus.cpu_write(addr, self.Y);
            }
            Opcode::BIT => {
                let v = self.bus.cpu_read(addr);
                let r = self.A & (v as u8);
                self.flags.Z = r == 0;
                self.flags.O = (v & (1 << 6)) != 0;
                self.flags.N = (v & (1 << 7)) != 0;
            }
            Opcode::PHP => {
                self.push_flags();
            }
            Opcode::PLP => {
                let flag = self.pop();
                self.flags.set_byte(flag);
            }
            Opcode::PHA => {
                self.push(self.A);
            }
            Opcode::PLA => {
                self.A = self.pop();
                self.flags.set_zn(self.A);
            }
            Opcode::CMP => {
                let v = self.bus.cpu_read(addr);
                self.flags.C = self.A >= v;
                self.flags.set_zn(self.A.wrapping_sub(v));
            }
            Opcode::CPX => {
                let v = self.bus.cpu_read(addr);
                self.flags.C = self.X >= v;
                self.flags.set_zn(self.X.wrapping_sub(v));
            }
            Opcode::CPY => {
                let v = self.bus.cpu_read(addr);
                self.flags.C = self.Y >= v;
                self.flags.set_zn(self.Y.wrapping_sub(v));
            }
            Opcode::AND => {
                let v = self.bus.cpu_read(addr);
                self.A = self.A & v;
                self.flags.set_zn(self.A);
            }
            Opcode::ORA => {
                let v = self.bus.cpu_read(addr);
                self.A = self.A | v;
                self.flags.set_zn(self.A);
            }
            Opcode::EOR => {
                let v = self.bus.cpu_read(addr);
                self.A = self.A ^ v;
                self.flags.set_zn(self.A);
            }
            Opcode::ADC => {
                let v = self.bus.cpu_read(addr) as u16;
                let res: u16 = (self.A as u16) + v + (self.flags.C as u16);
                self.flags.C = res > 0xFF;
                self.flags.O = !((self.A as u16) ^ v) & ((self.A as u16) ^ res) & 0x80 != 0;
                self.A = res as u8;
                self.flags.set_zn(self.A);
            }
            Opcode::SBC => {
                let mut v = self.bus.cpu_read(addr) as u16;
                v = v ^ 0x00FF;
                let res: u16 = (self.A as u16) + v + (self.flags.C as u16);
                self.flags.C = res > 0xFF;
                self.flags.O = !((self.A as u16) ^ v) & ((self.A as u16) ^ res) & 0x80 != 0;
                self.A = res as u8;
                self.flags.set_zn(self.A);
            }
            Opcode::INY => {
                self.Y = self.Y.wrapping_add(1);
                self.flags.set_zn(self.Y);
            }
            Opcode::INX => {
                self.X = self.X.wrapping_add(1);
                self.flags.set_zn(self.X);
            }
            Opcode::DEX => {
                self.X = self.X.wrapping_sub(1);
                self.flags.set_zn(self.X);
            }
            Opcode::DEY => {
                self.Y = self.Y.wrapping_sub(1);
                self.flags.set_zn(self.Y);
            }
            Opcode::TAY => {
                self.Y = self.A;
                self.flags.set_zn(self.Y);
            }
            Opcode::TAX => {
                self.X = self.A;
                self.flags.set_zn(self.X);
            }
            Opcode::TYA => {
                self.A = self.Y;
                self.flags.set_zn(self.A);
            }
            Opcode::TXA => {
                self.A = self.X;
                self.flags.set_zn(self.A);
            }
            Opcode::TSX => {
                self.X = self.SP;
                self.flags.set_zn(self.X);
            }
            Opcode::TXS => {
                self.SP = self.X;
            }
            Opcode::INC => {
                let mut v = self.bus.cpu_read(addr);
                v = v.wrapping_add(1);
                self.flags.set_zn(v);
                self.bus.cpu_write(addr, v);
            }
            Opcode::DEC => {
                let mut v = self.bus.cpu_read(addr);
                v = v.wrapping_sub(1);
                self.flags.set_zn(v);
                self.bus.cpu_write(addr, v);
            }
            Opcode::RTI => {
                let flag = self.pop();
                self.flags.set_byte(flag);
                self.PC = self.pop_u16();
            }
            Opcode::LSR | Opcode::ROR => {
                let data = if ins.mode == AddressingMode::ACC {
                    addr
                } else {
                    self.bus.cpu_read(addr) as u16
                };

                let prev_c = self.flags.C;
                self.flags.C = (data & 0x0001) != 0;
                let mut res = (data >> 1) as u8;
                if ins.opcode == Opcode::ROR && prev_c {
                    res = res | 1 << 7;
                }
                self.flags.set_zn(res);

                if ins.mode == AddressingMode::ACC {
                    self.A = res;
                } else {
                    self.bus.cpu_write(addr, res);
                }
            }
            Opcode::ASL | Opcode::ROL => {
                let data = if ins.mode == AddressingMode::ACC {
                    addr
                } else {
                    self.bus.cpu_read(addr) as u16
                };

                let prev_c = self.flags.C;
                self.flags.C = (data & 0x80) != 0;
                let mut res = (data << 1) as u8;
                if ins.opcode == Opcode::ROL && prev_c {
                    res = res | 1;
                }
                self.flags.set_zn(res);

                if ins.mode == AddressingMode::ACC {
                    self.A = res;
                } else {
                    self.bus.cpu_write(addr, res);
                }
            }
            Opcode::BRK => {
                self.PC += 1;
                self.flags.I = true;
                self.push_u16(self.PC);

                self.push_flags();

                self.PC = self.read_from_location_u16(0xFFFE);
            }
            Opcode::ERR => {}
        }

        self.cycles -= 1;
    }

    fn branch_jump(&mut self, addr: u16) {
        self.cycles += 1;

        // Different page? +1 cycle
        if (addr & 0xFF00) != (self.PC & 0xFF00) {
            self.cycles += 1;
        }

        self.PC = addr;
    }

    fn push_flags(&mut self) {
        let mut st = self.flags.to_byte();
        st = st | (1 << 5);
        st = st | (1 << 4);
        self.push(st);
    }

    fn push(&mut self, v: u8) {
        self.bus.cpu_write(0x0100 + self.SP as u16, v);
        self.SP -= 1;
    }

    fn push_u16(&mut self, v: u16) {
        self.push((v >> 8) as u8);
        self.push((v & 0x00FF) as u8);
    }

    fn pop(&mut self) -> u8 {
        self.SP += 1;
        return self.bus.cpu_read(0x0100 + self.SP as u16);
    }

    fn pop_u16(&mut self) -> u16 {
        let lo = self.pop();
        let hi = self.pop();
        to_u16(hi, lo)
    }

    fn read_addr(&mut self, ins: &Instruction) -> u16 {
        match ins.mode {
            AddressingMode::ABS => return self.read_addr_abs(),
            AddressingMode::ABY => return self.read_addr_aby(ins.opcode != Opcode::STA),
            AddressingMode::ABX => return self.read_addr_abx(ins.opcode != Opcode::STA),
            AddressingMode::IND => return self.read_addr_ind(),
            AddressingMode::IZX => return self.read_addr_izx(),
            AddressingMode::IZY => return self.read_addr_izy(),
            AddressingMode::IMM => return self.read_addr_imm(),
            AddressingMode::IMP => return self.read_addr_imp(),
            AddressingMode::ACC => return self.read_addr_acc(),
            AddressingMode::ZP0 => return self.read_addr_zp0(),
            AddressingMode::ZPX => return self.read_addr_zpx(),
            AddressingMode::ZPY => return self.read_addr_zpy(),
            AddressingMode::REL => return self.read_addr_rel(),
        }
    }

    fn read_from_location(&mut self, v: u8) -> u16 {
        return to_u16(
            self.bus.cpu_read(v.wrapping_add(1) as u16),
            self.bus.cpu_read(v as u16),
        );
    }

    fn read_from_location_u16(&mut self, v: u16) -> u16 {
        return to_u16(
            self.bus.cpu_read(v.wrapping_add(1) as u16),
            self.bus.cpu_read(v as u16),
        );
    }

    fn read_addr_abs(&mut self) -> u16 {
        let lo = self.bus.cpu_read(self.PC);
        self.PC += 1;
        let hi = self.bus.cpu_read(self.PC);
        self.PC += 1;
        return to_u16(hi, lo);
    }

    fn read_addr_abx(&mut self, cross_page_check: bool) -> u16 {
        let addr = self.read_addr_abs();
        let off = addr.wrapping_add(self.X as u16);
        if cross_page_check && off & 0xFF00 != addr & 0xFF00 {
            self.cycles += 1;
        }
        return off;
    }

    fn read_addr_aby(&mut self, cross_page_check: bool) -> u16 {
        let addr = self.read_addr_abs();
        let off = addr.wrapping_add(self.Y as u16);
        if cross_page_check && off & 0xFF00 != addr & 0xFF00 {
            self.cycles += 1;
        }
        return off;
    }

    fn read_addr_ind(&mut self) -> u16 {
        let addr = self.read_addr_abs();

        // NES had a bug on page boundary -> simulate it:
        if addr & 0x00FF == 0x00FF {
            return to_u16(self.bus.cpu_read(addr & 0xFF00), self.bus.cpu_read(addr));
        }

        return to_u16(self.bus.cpu_read(addr + 1), self.bus.cpu_read(addr));
    }

    fn read_addr_izx(&mut self) -> u16 {
        let mut addr = self.bus.cpu_read(self.PC);
        self.PC += 1;

        addr = addr.wrapping_add(self.X);

        return self.read_from_location(addr);
    }

    fn read_addr_izy(&mut self) -> u16 {
        let addr = self.bus.cpu_read(self.PC);
        self.PC += 1;

        let x = self.read_from_location(addr);
        let off = x.wrapping_add(self.Y as u16);
        if off & 0xFF00 != x & 0xFF00 {
            self.cycles += 1;
        }
        return off;
    }

    fn read_addr_imm(&mut self) -> u16 {
        let x = self.PC;
        self.PC += 1;
        return x;
    }

    fn read_addr_imp(&mut self) -> u16 {
        return 0;
    }

    fn read_addr_acc(&mut self) -> u16 {
        return self.A as u16;
    }

    fn read_addr_zp0(&mut self) -> u16 {
        let x = self.bus.cpu_read(self.PC);
        self.PC += 1;
        return x as u16;
    }

    fn read_addr_zpx(&mut self) -> u16 {
        let mut x = (self.bus.cpu_read(self.PC) as u16) + (self.X as u16);
        x = x & 0x00FF;
        self.PC += 1;
        return x;
    }

    fn read_addr_zpy(&mut self) -> u16 {
        let mut x = (self.bus.cpu_read(self.PC) as u16) + (self.Y as u16);
        x = x & 0x00FF;
        self.PC += 1;
        return x;
    }

    fn read_addr_rel(&mut self) -> u16 {
        let x = self.bus.cpu_read(self.PC);

        self.PC += 1;
        return ((self.PC as i16) + (x as i8) as i16) as u16;
    }
}
