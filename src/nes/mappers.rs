pub trait Mapper {
    fn map_write(&mut self, addr: u16, data: u8) -> u16;
    fn map_read(&mut self, addr: u16) -> u16;
    fn map_ppu_write(&mut self, addr: u16) -> u16;
    fn map_ppu_read(&mut self, addr: u16) -> u16;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Mapper0 {
    one_bank: bool,
}

impl Mapper for Mapper0 {
    fn map_write(&mut self, _: u16, _: u8) -> u16 {
        panic!("write is not supported");
    }

    fn map_read(&mut self, addr: u16) -> u16 {
        let mut mapped_addr = addr - 0x8000;
        if self.one_bank {
            mapped_addr &= 0x3fff;
        }
        mapped_addr
    }

    fn map_ppu_write(&mut self, addr: u16) -> u16 {
        addr
    }

    fn map_ppu_read(&mut self, addr: u16) -> u16 {
        addr
    }
}

impl Mapper0 {
    pub fn new(rom_pages: u8) -> Self {
        Self {
            one_bank: rom_pages == 1,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Mapper3 {
    one_bank: bool,
    bank_select: u16,
}

impl Mapper for Mapper3 {
    fn map_write(&mut self, addr: u16, data: u8) -> u16 {
        self.bank_select = (data & 0x03) as u16;
        addr
    }

    fn map_read(&mut self, addr: u16) -> u16 {
        let mut mapped_addr = addr - 0x8000;
        if self.one_bank {
            mapped_addr &= 0x3fff;
        }
        mapped_addr
    }

    fn map_ppu_write(&mut self, addr: u16) -> u16 {
        addr
    }

    fn map_ppu_read(&mut self, addr: u16) -> u16 {
        addr | (self.bank_select << 13)
    }
}

impl Mapper3 {
    pub fn new(rom_pages: u8) -> Self {
        Self {
            one_bank: rom_pages == 1,
            bank_select: 0,
        }
    }
}
