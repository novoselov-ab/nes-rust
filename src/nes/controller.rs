use super::bus::CpuBusDevice;
use std::ops::Range;

// Support only one for now

pub struct Controller {
    pub input: u8,
    pub state: u8,
    pub num: u16,
}

impl CpuBusDevice for Controller {
    fn get_addr_range(&self) -> &Range<u16> {
        if self.num == 0 {
            &(0x4016..0x4017)
        } else {
            &(0x4017..0x4018)
        }
    }

    fn cpu_write(&mut self, _: u16, _: u8) {
        self.state = self.input;
    }

    fn cpu_read(&mut self, _: u16) -> u8 {
        let data = ((self.state & 0x80) > 0) as u8;
        self.state <<= 1;
        data
    }
}

impl Controller {
    pub fn new(num: u16) -> Self {
        Controller {
            input: 0,
            state: 0,
            num: num,
        }
    }
}
