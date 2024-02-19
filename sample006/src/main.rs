fn main() {
    // struct
    let rect: Rectangle = Rectangle {
        width: 10,
        height: 11
    };
    println!("Rectangle is a struct: {},{}", rect.width, rect.height);

    // tuple
    let tuple: (u32, u32) = (5, 6);
    println!("Just a tuple: {}, {}", tuple.0, tuple.1);

    // tuple struct
    let point = Point(1, 2);
    println!("Point is tuple struct: {}, {}", point.0, point.1);
}

struct Rectangle {
    pub width: u32,
    pub height: u32,
}

struct Point(u32, u32);
