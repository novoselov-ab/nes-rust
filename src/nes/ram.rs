use super::bus::BusDevice;
use std::ops::Range;

const RAM_SIZE: u16 = 0x800;
const RAM_RANGE: Range<u16> = 0x0000..0x1FFF;

pub struct Ram {
    pub bytes: Vec<u8>,
}

impl BusDevice for Ram {
    fn get_addr_range(&self) -> &Range<u16> {
        &RAM_RANGE
    }

    fn write(&mut self, addr: u16, data: u8) {
        self.bytes[(addr & (RAM_SIZE - 1)) as usize] = data;
    }

    fn read(&mut self, addr: u16) -> u8 {
        return self.bytes[(addr & (RAM_SIZE - 1)) as usize];
    }
}

impl Ram {
    pub fn new() -> Self {
        Ram {
            bytes: vec![0; RAM_SIZE as usize],
        }
    }
}
