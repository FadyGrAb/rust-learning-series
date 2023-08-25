#[derive(Debug)]
struct Car {
    maker: String,
    model: String,
    year_of_making: u16,
    max_speed_kph: u16,
}
fn main() {
    let my_car = Car {
        maker: String::from("Maker1"),
        model: String::from("Model1"),
        year_of_making: 2023,
        max_speed_kph: dbg!(4 * 50),
    };

    println!("{my_car:?}");
    println!("{my_car:#?}");
    dbg!(&my_car);
}
