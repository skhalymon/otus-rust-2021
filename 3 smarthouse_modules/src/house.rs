use crate::errors::SmartHouseError;
use crate::room::Room;
use crate::traits::Report;
#[derive(Clone)]
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
            Ok(self.rooms.len() - 1)
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
        println!("Report called from House");
        for room in self.rooms.iter() {
            room.report()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_hose() {
        let house = House::new("Test House".into());
        assert_eq!(house.name, "Test House".to_string())
    }
    #[test]
    fn room_management() {
        let mut house = House::new("Test House".into());
        let room = Room::new("Test Room".into(), 40);
        let room2 = room.clone();
        if let Ok(r) = house.add_room(room) {
            assert_eq!(0, r)
        };
        // room duplicate
        let r = house.add_room(room2);
        assert!(r.is_err());

        let removed = house.remove_room("Test Room".into());
        assert!(removed);
        assert_eq!(house.rooms().len(), 0);
    }
}
