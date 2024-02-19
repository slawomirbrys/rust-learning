use std::fmt::Display;

fn main() {
    let color = Color::RGB(1, 2, 3);
    println!("Color is {}", color.to_string());

    let color = Color::RGB(0.1, 0.2, 0.3);
    println!("Color is {}", color.to_string());
}


enum Color<C> {
    RGB(C, C, C),
    #[allow(dead_code)]
    HSV(C, C, C),
    #[allow(dead_code)]
    Pantone(String)
}

impl<C: Display> Color<C> {
    fn to_string(&self) -> String {
        match self {
            Color::RGB(r, g, b) => format!("rgb({}, {}, {})", r, g, b),
            Color::HSV(h, s, v) => format!("hsv({}, {}, {})", h, s, v),
            Color::Pantone(name) => name.to_string()
        }
    }
}