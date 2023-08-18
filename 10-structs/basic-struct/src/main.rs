struct Car {
    maker: String,
    model: String,
    year_of_making: u32,
    color: String,
}

fn main() {
    // Define and using a Struct
    let car1 = Car {
        model: String::from("model1"),
        maker: String::from("Maker1"),
        color: String::from("Red"),
        year_of_making: 2023,
    };

    println!("Car 1 is a {} and of {} color", car1.maker, car1.color);

    let car2 = get_car(
        String::from("Maker2"),
        String::from("Model2"),
        2022,
        String::from("Yellow"),
    );

    println!("Car 2 is a {} of {} color", car2.maker, car2.color);

    let mut car3 = Car {
        color: String::from("Blue"),
        ..car1
    };

    // println!("Can we print car1 maker now? {}", car1.maker);  // ERROR: borrow of moved value: `car1.maker`

    println!("Car 3 is a {} and of {} color", car3.maker, car3.color);

    car3.color = String::from("Green");

    println!("Now car 3 is of {} color", car3.color);
}

fn get_car(maker: String, model: String, year_of_making: u32, color: String) -> Car {
    Car {
        maker,
        model,
        year_of_making,
        color,
    }
}
