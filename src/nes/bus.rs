use std::cell::RefCell;
use std::ops::Range;
use std::rc::Rc;

struct DeviceConnection {
    device: Rc<RefCell<dyn CpuBusDevice>>,
    addr_range: Range<u16>,
}

pub struct Bus {
    connections: Vec<DeviceConnection>,
}

pub trait CpuBusDevice {
    fn get_addr_range(&self) -> &Range<u16>;

    fn cpu_write(&mut self, addr: u16, data: u8);
    fn cpu_read(&mut self, addr: u16) -> u8;
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            connections: vec![],
        }
    }

    pub fn cpu_write(&mut self, addr: u16, data: u8) {
        for connection in &mut self.connections {
            if connection.addr_range.contains(&addr) {
                let mut device = connection.device.borrow_mut();
                device.cpu_write(addr, data);
                return;
            }
        }

        //panic!("no device with range: {} to write to.", addr);
    }

    pub fn cpu_read(&mut self, addr: u16) -> u8 {
        for connection in &mut self.connections {
            if connection.addr_range.contains(&addr) {
                let mut device = connection.device.borrow_mut();
                return device.cpu_read(addr);
            }
        }
        //panic!("no device with range: {} to read from.", addr);

        0
    }

    pub fn connect(&mut self, device: Rc<RefCell<dyn CpuBusDevice>>) {
        let addr_range = device.borrow_mut().get_addr_range().clone();
        self.connections.push(DeviceConnection {
            device: device,
            addr_range: addr_range,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::super::ram::Ram;
    use super::*;

    #[test]
    fn bus_devices() {
        let mut b = Bus::new();

        let r1 = Rc::new(RefCell::new(Ram::new()));
        b.connect(r1.clone());

        b.cpu_write(25, 16);

        assert_eq!(r1.borrow_mut().bytes[5], 0);
        assert_eq!(r1.borrow_mut().bytes[25], 16);

        assert_eq!(b.cpu_read(25), 16);
        assert_eq!(b.cpu_read(24), 0);
        assert_eq!(b.cpu_read(5), 0);
    }
}
