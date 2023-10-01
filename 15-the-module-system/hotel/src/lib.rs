mod reception {
    pub mod booking {
        pub fn book_room() {
            println!("Booking room!")
        }
        pub fn cancel_booking() {}
    }

    pub mod guests_management {
        pub fn guest_checkin() {
            println!("Checking in!")
        }
        pub fn guest_checkout() {}
        fn receive_guest_request() {}

        pub fn get_room_number() -> i8 {
            10
        }
    }
    #[derive(Debug)]
    pub struct Room {
        pub view: String,
        pub beds_count: i8,
        number: i8,
    }
    impl Room {
        pub fn new(view: &str, beds: i8) -> Room {
            Room {
                view: String::from(view),
                beds_count: beds,
                number: guests_management::get_room_number(),
            }
        }
    }
}

mod facilities {
    mod house_keeping {
        fn clean_room() {}
        fn deliver_meal() {}
    }

    mod maintenance {
        fn pool_maintenance() {}
        fn electrical_maintenance() {}
        fn building_maintenance() {}
    }

    pub mod restaurants {
        pub fn prepare_tables() {
            println!("Preparing table")
        }
        pub fn make_meal() {
            println!("Making meal")
        }

        #[derive(Debug)]
        pub enum Meal {
            Breakfast,
            Brunch,
            Lunch,
            Diner,
        }
    }
}

pub mod guests {
    use super::reception::guests_management::guest_checkout;
    use crate::facilities::restaurants;
    // use super::facilities::restaurants;
    // use super::facilities::restaurants::*;
    // use super::facilities::restaurants::{make_meal, prepare_tables};

    fn book_a_room() {
        super::reception::booking::book_room();
        self::go_to_beach();
    }

    fn go_to_beach() {}
    fn go_to_pool() {}
    pub fn eat_meal() {
        restaurants::prepare_tables();
        restaurants::make_meal();
        let my_meal = restaurants::Meal::Diner;
        println!("Eating {my_meal:?}");
    }
    fn end_vacation() {
        guest_checkout();
        println!("Bye bye!");
    }
}

pub fn go_on_vacation() {
    crate::reception::booking::book_room(); // Absolute path
    reception::guests_management::guest_checkin(); // Relative path

    let my_room = reception::Room::new("Sea view", 2);

    // println!(
    //     "My room's vew is {} and my room number is {}. My room: {:?}",
    //     my_room.view, my_room.number, my_room
    // );

    println!("My room's vew is {}. My room: {:?}", my_room.view, my_room);
}
