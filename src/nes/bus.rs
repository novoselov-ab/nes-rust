use std::cell::RefCell;
use std::ops::Range;
use std::rc::Rc;

struct DeviceConnection {
    device: Rc<RefCell<dyn BusDevice>>,
    addr_range: Range<u16>,
}

pub struct Bus {
    connections: Vec<DeviceConnection>,
}

pub trait BusDevice {
    fn get_addr_range(&self) -> &Range<u16>;

    fn write(&mut self, addr: u16, data: u8);
    fn read(&mut self, addr: u16) -> u8;
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            connections: vec![],
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        for connection in &mut self.connections {
            if connection.addr_range.contains(&addr) {
                let mut device = connection.device.borrow_mut();
                device.write(addr, data);
                return;
            }
        }

        //panic!("no device with range: {} to write to.", addr);
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        for connection in &mut self.connections {
            if connection.addr_range.contains(&addr) {
                let mut device = connection.device.borrow_mut();
                return device.read(addr);
            }
        }
        //panic!("no device with range: {} to read from.", addr);

        0
    }

    pub fn connect(&mut self, device: Rc<RefCell<dyn BusDevice>>) {
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

        b.write(25, 16);

        assert_eq!(r1.borrow_mut().bytes[5], 0);
        assert_eq!(r1.borrow_mut().bytes[25], 16);

        assert_eq!(b.read(25), 16);
        assert_eq!(b.read(24), 0);
        assert_eq!(b.read(5), 0);
    }
}
