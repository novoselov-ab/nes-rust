// use rand::rngs::ThreadRng;
// use rand::Rng;
use std::path::PathBuf;

use std::cell::RefCell;
// use std::fs::File;
use std::io::Write;
use std::rc::Rc;

pub mod bus;
pub mod cartridge;
pub mod controller;
pub mod cpu;
pub mod disasm;
pub mod dma;
pub mod logger;
pub mod mappers;
pub mod ppu;
pub mod ram;

use cartridge::Cartridge;
use controller::Controller;
use cpu::Cpu;
use cpu::{to_u16, AddressingMode, INSTRUCTION_LOOKUP};
use dma::DmaDevice;
use logger::Logger;
use ppu::Ppu;
use ram::Ram;

#[derive(Default)]
pub struct FrameTime {
    pub dt: f32,
    pub dt_accum: f32,
    pub fps: f32,
}

/// NES main emulator
pub struct Emulator {
    pub cpu: Cpu,
    pub ppu: Rc<RefCell<Ppu>>,
    pub ram: Rc<RefCell<Ram>>,
    pub cartridge: Rc<RefCell<Cartridge>>,
    pub logger: Rc<RefCell<Logger>>,
    pub clock: i32,
    pub dma: Rc<RefCell<DmaDevice>>,
    pub controllers: [Rc<RefCell<Controller>>; 2],
    pub rom_loaded: bool,
    pub frame_time: FrameTime,
}

impl Emulator {
    pub fn new() -> Self {
        let mut cpu = Cpu::new();

        let ram = Rc::new(RefCell::new(Ram::new()));
        let cartridge = Rc::new(RefCell::new(Cartridge::new()));
        let logger = Rc::new(RefCell::new(Logger::new()));
        let ppu = Rc::new(RefCell::new(Ppu::new(cartridge.clone())));
        let dma = Rc::new(RefCell::new(DmaDevice::new()));
        let controller0 = Rc::new(RefCell::new(Controller::new(0)));
        let controller1 = Rc::new(RefCell::new(Controller::new(1)));

        cpu.bus.connect(ram.clone());
        cpu.bus.connect(cartridge.clone());
        cpu.bus.connect(logger.clone());
        cpu.bus.connect(ppu.clone());
        cpu.bus.connect(dma.clone());
        cpu.bus.connect(controller0.clone());
        cpu.bus.connect(controller1.clone());

        Emulator {
            cpu,
            ppu,
            ram,
            cartridge,
            logger,
            clock: 0,
            dma,
            controllers: [controller0, controller1],
            rom_loaded: false,
            frame_time: FrameTime::default(),
        }
    }

    pub fn load_rom(&mut self, romfile: &PathBuf) {
        self.cartridge.borrow_mut().load_from_file(romfile);
        self.cpu.reset();
        self.ppu.borrow_mut().reset();
        self.dma.borrow_mut().reset();
        self.clock = 1;
        self.rom_loaded = true;
    }

    #[allow(dead_code)]
    pub fn write_state(&mut self, f: &mut impl Write) {
        write!(f, "{:04X} ", self.cpu.PC).unwrap();
        disasm::disasm(f, &mut self.cpu.bus, self.cpu.PC);

        let ins_code = self.cpu.bus.cpu_read(self.cpu.PC);
        let ins = &INSTRUCTION_LOOKUP[ins_code as usize];
        let mut addr_str = String::new();
        match ins.mode {
            // JMP ABS
            AddressingMode::ABS => {
                addr_str = format!(
                    "${:04X} ",
                    to_u16(
                        self.cpu.bus.cpu_read(self.cpu.PC + 2),
                        self.cpu.bus.cpu_read(self.cpu.PC + 1)
                    )
                );
            }
            AddressingMode::ABX => {
                addr_str = format!(
                    "${:04X},X ",
                    to_u16(
                        self.cpu.bus.cpu_read(self.cpu.PC + 2),
                        self.cpu.bus.cpu_read(self.cpu.PC + 1)
                    )
                );
            }
            AddressingMode::ABY => {
                addr_str = format!(
                    "${:04X},Y ",
                    to_u16(
                        self.cpu.bus.cpu_read(self.cpu.PC + 2),
                        self.cpu.bus.cpu_read(self.cpu.PC + 1)
                    )
                );
            }
            AddressingMode::REL => {
                addr_str = format!(
                    "${:04X} ",
                    self.cpu.PC + self.cpu.bus.cpu_read(self.cpu.PC + 1) as u16 + 2
                );
            }
            AddressingMode::IND => {
                addr_str = format!(
                    "(${:04X}) ",
                    to_u16(
                        self.cpu.bus.cpu_read(self.cpu.PC + 2),
                        self.cpu.bus.cpu_read(self.cpu.PC + 1)
                    )
                );
            }
            AddressingMode::IMP => {}
            AddressingMode::ACC => {
                addr_str = format!("A ");
            }
            AddressingMode::IMM => {
                addr_str = format!("#${:02X} ", self.cpu.bus.cpu_read(self.cpu.PC + 1));
            }
            AddressingMode::ZP0 => {
                addr_str = format!("${:02X} ", self.cpu.bus.cpu_read(self.cpu.PC + 1));
            }
            AddressingMode::IZX => {
                addr_str = format!("(${:02X},X) ", self.cpu.bus.cpu_read(self.cpu.PC + 1));
            }
            AddressingMode::IZY => {
                addr_str = format!("(${:02X}),Y ", self.cpu.bus.cpu_read(self.cpu.PC + 1));
            }
            AddressingMode::ZPX => {
                addr_str = format!("${:02X},X ", self.cpu.bus.cpu_read(self.cpu.PC + 1));
            }
            AddressingMode::ZPY => {
                addr_str = format!("${:02X},Y ", self.cpu.bus.cpu_read(self.cpu.PC + 1));
            }
        }
        write!(f, "{:<20}", addr_str).unwrap();

        write!(f, "A:{:02X} ", self.cpu.A).unwrap();
        write!(f, "X:{:02X} ", self.cpu.X).unwrap();
        write!(f, "Y:{:02X} ", self.cpu.Y).unwrap();

        write!(f, "P:{:02X} ", self.cpu.flags.to_byte()).unwrap();
        write!(f, "SP:{:02X} ", self.cpu.SP).unwrap();
        write!(
            f,
            "PPU:{:>3},{:>3} ",
            self.ppu.borrow().scanline,
            self.ppu.borrow().cycle
        )
        .unwrap();
        write!(f, "CYC:{} ", self.cpu.total_cycles).unwrap();

        f.write_all(b"\n").unwrap();
    }

    pub fn update(&mut self, dt: f32) {
        if !self.rom_loaded {
            return;
        }

        // limit fps
        if !self.frame_time.update(dt) {
            return;
        }

        while !self.ppu.borrow().screen.complete {
            self.clock();
        }
    }

    pub fn clock(&mut self) {
        self.ppu.borrow_mut().clock();

        if self.clock % 3 == 0 {
            if self.dma.borrow_mut().transfer {
                self.dma
                    .borrow_mut()
                    .clock(self.clock, &mut self.cpu, &mut self.ppu.borrow_mut());
            } else {
                self.cpu.clock();
            }
        }

        if self.ppu.borrow().nmi {
            self.ppu.borrow_mut().nmi = false;
            self.cpu.nmi();
        }

        self.clock += 1;
    }
}

impl FrameTime {
    pub fn update(&mut self, dt: f32) -> bool {
        // Limit to 30 fps
        self.dt_accum += dt;
        self.dt += dt;
        const FRAME_TIME: f32 = 1.0 / 30.0;
        if self.dt_accum < FRAME_TIME {
            return false;
        }
        self.fps = 1.0 / self.dt;
        self.dt_accum -= FRAME_TIME;
        self.dt = 0.0;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::path::PathBuf;

    #[test]
    fn cpu_test() {
        let mut e = Emulator::new();

        let mut buf = Vec::new();

        let log_file = File::create(&PathBuf::from("nestest_out.log")).unwrap();

        e.load_rom(&PathBuf::from("roms/nestest.nes"));
        e.cpu.PC = 0xC000;
        let mut cmp_file = BufReader::new(File::open(&PathBuf::from("roms/nestest.log")).unwrap());
        for _ in 1..100000 {
            // After that some NOP operations comes in which we don't support
            if e.cpu.total_cycles >= 14579 {
                break;
            }

            if e.cpu.cycles == 0 {
                buf.clear();
                e.write_state(&mut buf);
                let out_line = String::from_utf8_lossy(&buf);
                write!(&log_file, "{}", out_line).unwrap();

                let mut cmp_line = String::new();
                cmp_file.read_line(&mut cmp_line).unwrap();

                // ignore whitespace
                let out_vec: Vec<&str> = out_line.split_whitespace().collect();
                let mut cmp_vec: Vec<&str> = cmp_line.split_whitespace().collect();

                // filter out some data we don't support:
                // find range from "=" to "A:" and remove
                let i1 = cmp_vec.iter().position(|&r| r.starts_with("A:")).unwrap();
                let p0 = cmp_vec
                    .iter()
                    .take(i1 + 1)
                    .position(|&r| r.starts_with("@"));
                let p1 = cmp_vec
                    .iter()
                    .take(i1 + 1)
                    .position(|&r| r.starts_with("="));
                let p = std::cmp::min(p0.unwrap_or(i1), p1.unwrap_or(i1));
                if p < i1 {
                    cmp_vec.drain(p..i1);
                }

                assert_eq!(out_vec, cmp_vec);
            }

            e.clock();
            e.clock();
            e.clock();
        }
    }
}
