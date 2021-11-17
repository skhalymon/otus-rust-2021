#![allow(dead_code)]

mod errors {
    use std::fmt::{Display, Formatter};

    #[derive(Debug)]
    pub enum SmartHouseError {
        AlreadyAdded(String),
        NotFound(String),
    }

    impl Display for SmartHouseError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let msg = match &self {
                Self::AlreadyAdded(name) => format!("Name {} already added", name),
                Self::NotFound(name) => format!("Name {} not found", name),
            };
            writeln!(f, "{}", msg)
        }
    }

    impl std::error::Error for SmartHouseError {}
}

mod traits {
    pub trait Report {
        fn report(&self);
    }
}
mod device {
    use crate::traits::Report;

    type Celcious = i8;
    type KilowattPerHour = i8;

    struct SmartSocket {
        turned_on: bool,
    }

    impl SmartSocket {
        fn turn_on(&mut self) {
            self.turned_on = true;
        }
        fn turn_off(&mut self) {
            self.turned_on = false;
        }
        fn current_power_usage(&self) -> KilowattPerHour {
            todo!()
        }
    }

    struct Thermometer;

    impl Thermometer {
        pub fn temperature(&self) -> Celcious {
            todo!()
        }
    }

    pub enum DeviceKind {
        SmartSocket,
        Thermometer,
    }

    pub struct Device {
        pub name: String,
        pub kind: DeviceKind,
        pub description: String,
    }

    impl Device {
        pub fn new(name: String, kind: DeviceKind, description: String) -> Device {
            Device {
                name,
                kind,
                description,
            }
        }
    }
    impl Report for Device {
        fn report(&self) {
            println!("Report called from Device")
        }
    }
}
mod room {
    use crate::device::Device;
    use crate::errors::SmartHouseError;
    use crate::traits::Report;

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
        pub fn devices(&self) -> &Vec<Device> {
            &self.devices
        }
        pub fn add_device(&mut self, device: Device) -> Result<usize, SmartHouseError> {
            if let Some(d) = self.devices.iter().find(|d| d.name == device.name) {
                Err(SmartHouseError::AlreadyAdded(d.name.clone()))
            } else {
                self.devices.push(device);
                Ok(self.devices.len())
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
            println!("Report called from Room")
        }
    }
}
mod house {
    use crate::errors::SmartHouseError;
    use crate::room::Room;
    use crate::traits::Report;

    pub struct House {
        name: String,
        rooms: Vec<Room>,
    }

    impl House {
        pub fn new(name: String) -> House {
            House {
                name,
                rooms: vec![],
            }
        }
        pub fn rooms(&self) -> &Vec<Room> {
            &self.rooms
        }
        pub fn add_room(&mut self, room: Room) -> Result<usize, SmartHouseError> {
            if let Some(r) = self.rooms.iter().find(|r| r.name == room.name) {
                Err(SmartHouseError::AlreadyAdded(r.name.clone()))
            } else {
                self.rooms.push(room);
                Ok(self.rooms.len())
            }
        }
        pub fn remove_room(&mut self, room_name: String) -> bool {
            if let Some(index) = self.rooms.iter().position(|r| r.name == room_name) {
                self.rooms.remove(index);
                true
            } else {
                false
            }
        }
    }

    impl Report for House {
        fn report(&self) {
            println!("Report called from House")
        }
    }
}
#[cfg(test)]
mod tests {
    #![allow(unused_must_use)]
    use crate::device::{Device, DeviceKind};
    use crate::house::House;
    use crate::room::Room;
    use crate::traits::Report;

    #[test]
    fn smoke_smart_home() {
        let mut house = House::new("My Home".to_owned());
        let mut room = Room::new("Bedroom".to_owned(), 23);
        let socket = Device::new(
            "Near bra".to_owned(),
            DeviceKind::SmartSocket,
            "Some socket near bra".to_owned(),
        );
        room.add_device(socket);
        house.add_room(room);
        house.report();
    }
}
