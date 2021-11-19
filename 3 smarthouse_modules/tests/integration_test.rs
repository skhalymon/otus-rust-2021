use smarthouse_modules::prelude::*;

#[test]
fn smoke_smart_home() {
    let mut house = House::new("My Home".into());
    let mut room = Room::new("Bedroom".into(), 23);
    let socket = Device::new_smart_socket("Near bra".into(), "Some socket near bra".into());
    if let Err(e) = room.add_device(socket) {
        println!("Failed to add device {}", e)
    };

    if let Err(e) = house.add_room(room) {
        println!("Failed to add room {}", e)
    };

    house.report();
}
