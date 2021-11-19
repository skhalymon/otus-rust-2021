mod device_kind;

use crate::traits::Report;
pub use device_kind::DeviceKind;

#[derive(Clone)]
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
    pub fn new_smart_socket(name: String, description: String) -> Device {
        Device {
            name,
            kind: DeviceKind::SmartSocket,
            description,
        }
    }
    pub fn new_thermometer(name: String, description: String) -> Device {
        Device {
            name,
            kind: DeviceKind::Thermometer,
            description,
        }
    }
}
impl Report for Device {
    fn report(&self) {
        println!("Report called from Device")
    }
}
