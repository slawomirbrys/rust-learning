fn main() {
    let s1 = Rectangle {
        width: 15f64,
        height: 7.34
    };
    println!("Shape is Rectangle w={} x h={}", s1.width, s1.height);
    print_area(&s1);
    print_area_dyn(&s1);

    let s2 = Circle {
        radius: 4_f64
    };
    println!("Shape is Circle r={}", s2.radius);
    print_area(&s2);
    print_area_dyn(&s2);

    let s3 = Square {
       width: 17.66
    };
    println!("Shape is Square w={}", s3.width);

    // square doesn't implement this trait
    //print_area(&s3);
    //print_area_dyn(&s3);
    // but has its own implementation
    println!("Area custom: {}", s3.get_area());
}

fn print_area<S: Shape>(shape: &S) {
    println!("Area: {}", shape.get_area())
}

fn print_area_dyn(shape:&dyn Shape) {
    println!("Area dyn: {}", shape.get_area());
}


trait Shape {
    fn get_area(&self) -> f64;
}

struct Rectangle {
    pub width: f64,
    pub height: f64
}

impl Shape for Rectangle {
    fn get_area(&self) -> f64 {
        self.width * self.height
    }
}

struct Circle {
    pub radius: f64
}

impl Shape for Circle {
    fn get_area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}


struct Square {
    pub width: f64
}

impl Square {
    fn get_area(&self) -> f64 {
        self.width * self.width
    }
}