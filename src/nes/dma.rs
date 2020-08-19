use super::bus::BusDevice;
use super::cpu::Cpu;
use super::ppu::Ppu;
use std::ops::Range;

// Support only one for now

pub struct DmaDevice {
    pub page: u8,
    pub addr: u8,
    pub data: u8,
    pub flag: bool,
    pub transfer: bool,
}

impl DmaDevice {
    pub fn new() -> Self {
        DmaDevice {
            page: 0,
            addr: 0,
            data: 0,
            flag: true,
            transfer: false,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn clock(&mut self, clock: i32, cpu: &mut Cpu, ppu: &mut Ppu) {
        if self.flag {
            if clock % 2 == 1 {
                self.flag = false;
            }
        } else {
            if clock % 2 == 0 {
                self.data = cpu.bus.cpu_read((self.page as u16) << 8 | (self.addr as u16));
            } else {
                ppu.write_oam(self.addr, self.data);
                self.addr = self.addr.wrapping_add(1);
                if self.addr == 0x00 {
                    self.transfer = false;
                    self.flag = true;
                }
            }
        }
    }
}

impl BusDevice for DmaDevice {
    fn get_addr_range(&self) -> &Range<u16> {
        &(0x4014..0x4015)
    }

    fn cpu_write(&mut self, _: u16, data: u8) {
        self.page = data;
        self.addr = 0x00;
        self.transfer = true;
    }

    fn cpu_read(&mut self, _: u16) -> u8 {
        0
    }
}
