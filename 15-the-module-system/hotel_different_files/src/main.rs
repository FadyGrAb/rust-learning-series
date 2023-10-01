use hotel_different_files::reception;

fn main() {
    let my_room = reception::Room::get_room("pool view", 1);
    println!("My room is {my_room:?}");
}
