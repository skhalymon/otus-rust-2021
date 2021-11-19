use crate::types::{Celcious, KilowattPerHour};

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

#[non_exhaustive]
#[derive(Clone)]
pub enum DeviceKind {
    SmartSocket,
    Thermometer,
}
