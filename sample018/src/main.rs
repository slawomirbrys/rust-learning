use std::string::ToString;

fn main() {
    let rect = Rectangle::new(10, 20);
    println!("This rectangle is {}", rect.to_string());
}

struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width: width,
            height: height,
        }
    }

    pub fn to_string(&self) -> String {
        format!("[{},{}]", self.width, self.height)
    }
}

impl Drop for Rectangle {
    fn drop(&mut self) {
        println!("Rectangle has been deallocated");
    }
}