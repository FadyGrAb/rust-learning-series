struct Rectangle {
    height: u32,
    width: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width >= other.width) & (self.height >= other.height)
    }
    fn square(dimension: u32) -> Rectangle {
        Rectangle {
            width: dimension,
            height: dimension,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        height: 10,
        width: 5,
    };
    println!("rect1 is {} x {}", rect1.width, rect1.height);
    println!("rect1 area is: {}", rect1.area());

    let rect2 = Rectangle {
        height: 9,
        width: 4,
    };

    let rect3 = Rectangle {
        height: 11,
        width: 6,
    };

    println!("Can rect1 hold rect2: {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3: {}", rect1.can_hold(&rect3));

    let square1 = Rectangle::square(5);

    println!("square1 is {} x {}", square1.width, square1.height);
}
