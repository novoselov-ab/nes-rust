use super::bus::BusDevice;
use std::fs;
use std::ops::Range;
use std::path::PathBuf;

use super::mappers::{Mapper, Mapper0, Mapper3};

pub struct Cartridge {
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    prg_ram: Vec<u8>,
    mapper: Box<dyn Mapper>,
    vertical_mirror: bool,
}

impl BusDevice for Cartridge {
    fn get_addr_range(&self) -> &Range<u16> {
        &(0x8000..0xFFFF)
    }

    fn write(&mut self, addr: u16, data: u8) {
        self.mapper.map_write(addr, data);
    }

    fn read(&mut self, addr: u16) -> u8 {
        let mapped_addr = self.mapper.map_read(addr);
        self.prg_rom[(mapped_addr as usize)]
    }
}

impl Cartridge {
    pub fn new() -> Self {
        Cartridge {
            prg_rom: vec![],
            chr_rom: vec![],
            prg_ram: vec![],
            mapper: Box::new(Mapper0::new(0)),
            vertical_mirror: false,
        }
    }

    pub fn ppu_write(&mut self, addr: u16, data: u8) {
        let mapped_addr = self.mapper.map_ppu_write(addr);
        self.chr_rom[(mapped_addr as usize)] = data;
    }

    pub fn ppu_read(&mut self, addr: u16) -> u8 {
        let mapped_addr = self.mapper.map_ppu_read(addr) as usize;
        if mapped_addr >= self.chr_rom.len() {
            return 0;
        }
        self.chr_rom[mapped_addr]
    }

    pub fn is_vertical_mirror(&self) -> bool {
        // TODO: add mapper?
        self.vertical_mirror
    }

    pub fn load_from_file(&mut self, romfile: &PathBuf) {
        // Load ROM from file
        let contents = match fs::read(romfile) {
            Err(e) => {
                println!("Can't read file: '{0}'. Error: {1}", romfile.display(), e);
                return;
            }
            Ok(f) => f,
        };

        let mut next = 0;

        //////////////////////////////////////////////
        // 16 byte      Header
        //////////////////////////////////////////////

        // 00h  File ID ("NES",1Ah) (aka 4Eh,45h,53h,1Ah)
        if contents[0..4] != ['N' as u8, 'E' as u8, 'S' as u8, 0x1A] {
            println!("Wrong file ID in nes file: '{0}'.", romfile.display());
            return;
        }

        // 04h  Number of 16K PRG-ROM pages
        let rom_pages = contents[4];

        // 05h  Number of 8K CHR-ROM pages (00h=None / VRAM)
        let chr_pages = contents[5];

        // 06h  Cartridge Type LSB
        // Bit7-4  Mapper Number (lower 4bits)
        // Bit3    1=Four-screen VRAM layout
        // Bit2    1=512-byte trainer/patch at 7000h-71FFh
        // Bit1    1=Battery-backed SRAM at 6000h-7FFFh, set only if battery-backed
        // Bit0    0=Horizontal mirroring, 1=Vertical mirroring
        let type_lsb = contents[6];
        let has_trainer = (type_lsb & (1 << 2)) != 0;
        self.vertical_mirror = (type_lsb & (1 << 0)) != 0;

        // 07h  Cartridge Type MSB (ignore this and further bytes if Byte 0Fh nonzero)
        // Bit7-4  Mapper Number (upper 4bits)
        // Bit3-2  Reserved (zero)
        // Bit1    1=PC10 game (arcade machine with additional 8K Z80-ROM) (*)
        // Bit0    1=VS Unisystem game (arcade machine with different palette)
        let type_msb = contents[7];

        // Read mapper
        let mapper_number = ((type_lsb >> 4) & 0xf) | (type_msb & 0xf0);

        // 08h  Number of 8K RAM (SRAM?) pages (usually 00h=None-or-not-specified)
        let ram_pages = contents[8];

        next += 16; // Header size

        //////////////////////////////////////////////
        // 512 byte      Trainer
        //////////////////////////////////////////////

        if has_trainer {
            next += 512;
        }

        //////////////////////////////////////////////
        // N*16K        PRG-ROM
        //////////////////////////////////////////////
        {
            let rom_size = (rom_pages as usize) * 0x4000;
            self.prg_rom = contents[next..next + rom_size].to_vec();
            println!("rom_size: {:?}", rom_size);
            next += rom_size;
        }

        //////////////////////////////////////////////
        // N*8K        CHR-ROM
        //////////////////////////////////////////////
        {
            let chr_size = (chr_pages as usize) * 0x2000;
            self.chr_rom = contents[next..next + chr_size].to_vec();
            println!("chr_size: {:?}", chr_size);
            //next += chr_size;
        }

        //////////////////////////////////////////////
        // N*8K        PRG-RAM
        //////////////////////////////////////////////
        {
            let ram_size = (ram_pages as usize) * 0x204C;
            self.prg_ram = vec![0; ram_size];
        }

        // Load mapper
        match mapper_number {
            0 => self.mapper = Box::new(Mapper0::new(rom_pages)),
            3 => self.mapper = Box::new(Mapper3::new(rom_pages)),
            _ => {
                println!("Unsupported mapper: {:?}", mapper_number);
                return;
            }
        }

        /*
        iNES Format (.NES)
        The overall file structure is, in following order:
          16 byte      Header
          512 byte     Trainer             ;-if any, see Byte 6, Bit2, mainly FFE games
          N*16K        PRG-ROM             ;-see Byte 4
          N*8K         CHR-ROM             ;-if any, see Byte 5
          8K       (*) PC10 INST-ROM       ;-if any, see Byte 7, Bit1
          16 byte  (*) PC10 PROM Data      ;-if any, see Byte 7, Bit1 ;\required, but
          16 byte  (*) PC10 PROM CounterOut;-if any, see Byte 7, Bit1 ;/often missing
          128 byte (*) Title               ;-if any (rarely used)
        Items marked as (*) are regulary used, but not offical part of the format.
        Many PC10 files declare Z80-ROM as additional VROM bank (instead Byte7/Bit1).
                */
    }
}
