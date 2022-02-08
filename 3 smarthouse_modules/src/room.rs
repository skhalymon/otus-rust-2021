use crate::device::Device;
use crate::errors::SmartHouseError;
use crate::traits::Report;

#[derive(Clone)]
pub struct Room {
    pub name: String,
    pub square: i32,
    devices: Vec<Device>,
}

impl Room {
    pub fn new(name: String, square: i32) -> Room {
        Room {
            name,
            square,
            devices: vec![],
        }
    }
    pub fn devices(&self) -> impl Iterator<Item = &Device> {
        self.devices.iter()
    }
    pub fn device(&self, device_name: String) -> Result<&Device, SmartHouseError> {
        if let Some(d) = self.devices.iter().find(|d| d.name == device_name) {
            Ok(d)
        } else {
            Err(SmartHouseError::NotFound(device_name))
        }
    }

    pub fn add_device(&mut self, device: Device) -> Result<usize, SmartHouseError> {
        if let Some(d) = self.devices.iter().find(|d| d.name == device.name) {
            Err(SmartHouseError::AlreadyAdded(d.name.clone()))
        } else {
            self.devices.push(device);
            // returns index of added device
            Ok(self.devices.len() - 1)
        }
    }
    pub fn remove_device(&mut self, device_name: String) -> bool {
        if let Some(index) = self.devices.iter().position(|d| d.name == device_name) {
            self.devices.remove(index);
            true
        } else {
            false
        }
    }
}
impl Report for Room {
    fn report(&self) {
        println!("Report called from Room");
        for device in self.devices.iter() {
            device.report()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_room() {
        let room = Room::new("Test Room".into(), 40);
        assert_eq!(room.name, "Test Room".to_string())
    }
    #[test]
    fn device_management() {
        let mut room = Room::new("Test Room".into(), 40);
        let device = Device::new_smart_socket("Test device".into(), "Test desc".into());
        let device2 = device.clone();
        if let Ok(r) = room.add_device(device) {
            assert_eq!(0, r)
        };
        // device duplicate
        let r = room.add_device(device2);
        assert!(r.is_err());

        let removed = room.remove_device("Test device".into());
        assert!(removed);
        assert_eq!(room.devices().count(), 0);
    }
}
