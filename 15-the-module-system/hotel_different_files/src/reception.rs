pub mod booking;
pub mod guests_management;

#[derive(Debug)]
pub struct Room {
    view: String,
    beds_count: i8,
    number: i8,
}
impl Room {
    pub fn get_room(view: &str, beds: i8) -> Room {
        Room {
            view: String::from(view),
            beds_count: beds,
            number: guests_management::get_room_number(),
        }
    }
}
