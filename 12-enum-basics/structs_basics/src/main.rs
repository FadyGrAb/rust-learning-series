enum SimpleAnswer {
    True,
    False,
}

struct UserAnswer {
    value: String,
    evaluation: SimpleAnswer,
}

enum Answer {
    True(String),
    False(String),
}

enum Shape {
    Rectangle { width: f32, height: f32 },
    Circle(f32),
    Square(f32),
}

impl Shape {
    fn area(&self) -> f32 {
        match self {
            self::Shape::Rectangle {
                width: w,
                height: h,
            } => w * h,
            self::Shape::Circle(radius) => radius * radius * 3.14,
            self::Shape::Square(side) => side * side,
        }
    }

    fn get_type(&self) -> &str {
        match self {
            self::Shape::Rectangle {
                width: _,
                height: _,
            } => "Rectangle",
            self::Shape::Circle(_) => "Circle",
            self::Shape::Square(_) => "Square",
        }
    }
}

fn main() {
    // Enum values
    let correct_answer = SimpleAnswer::True;
    let wrong_answer = SimpleAnswer::False;

    // Enums can be passed to functions
    evaluate_answer(correct_answer);
    evaluate_answer(wrong_answer);

    // Structs and Enums
    let true_with_value = UserAnswer {
        value: String::from("The sky is blue"),
        evaluation: SimpleAnswer::True,
    };

    let false_with_value = UserAnswer {
        value: String::from("The sky is brown"),
        evaluation: SimpleAnswer::False,
    };

    // Enums with values
    let true_with_value = Answer::True(String::from("The sky is blue"));
    let false_with_value = Answer::False(String::from("The sky is brown"));

    // Enums with methods
    let r = Shape::Rectangle {
        width: 10.0,
        height: 5.0,
    };
    let c = Shape::Circle(5.0);
    let s = Shape::Square(4.0);

    let shapes = [r, c, s];

    for shape in shapes.iter() {
        println!("{}: {}", shape.get_type(), shape.area())
    }
}

fn evaluate_answer(user_answer: SimpleAnswer) {}
