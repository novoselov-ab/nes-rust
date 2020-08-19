use super::bus::BusDevice;
use std::ops::Range;

const ADDR_RANGE: Range<u16> = 0x6000..0x7000;

pub struct Logger {
    pub bytes: Vec<u8>,
}

impl BusDevice for Logger {
    fn get_addr_range(&self) -> &Range<u16> {
        &ADDR_RANGE
    }

    fn cpu_write(&mut self, addr: u16, data: u8) {
        self.bytes[(addr - 0x6000) as usize] = data;
    }

    fn cpu_read(&mut self, _: u16) -> u8 {
        0
    }
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            bytes: vec![0; ADDR_RANGE.len() as usize],
        }
    }
}
