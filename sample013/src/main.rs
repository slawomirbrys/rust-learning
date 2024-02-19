fn main() {
    let r1 = Rectangle {
        width: 10,
        height: 20
    };
    println!("Area of rectangle 1 ({},{}) = {}", r1.width, r1.height, r1.get_area());

    let mut r2 = r1;
    r2.double_width();

    // moved 'r1' into 'r2', not a copy, so cannot use 'r1' anymore
    //println!("Area of rectangle 1 ({},{}) = {}", r1.width, r1.height, r1.get_area());
    println!("Area of rectangle 2 ({},{}) = {}", r2.width, r2.height, r2.get_area());

    let dim = r2.into_dimensions();
    println!("Dimensions of rectangle 2: {} x {}", dim.0, dim.1);

    // 'r2' ownership taken by `into_dimensions`
    //println!("{}", r2.height);
}


struct Rectangle {
    pub width: u32,
    pub height: u32
}

impl Rectangle {
    pub fn get_area(&self) -> u32 {
        self.width * self.height
    }

    pub fn double_width(&mut self) {
        self.width *= 2;
    }

    pub fn into_dimensions(self) -> (u32, u32) {
        (self.width, self.height)
    }
}