fn main() {
  let light_blue = Color::RGB(0.5, 0.5, 0.9);
    println!("{}", light_blue.to_string());
}


enum Color {
    RGB(f64, f64, f64),

    #[allow(dead_code)]
    HSV(f64, f64, f64),

    #[allow(dead_code)]
    Pantone(String)
}

impl Color {
    fn to_string(&self) -> String {
        match self {
            Color::RGB(r, g, b) => format!("rgb({}, {}, {})", r, g, b),
            Color::HSV(h, s, v) => format!("hsv({}, {}, {})", h, s, v),
            Color::Pantone(name) => name.to_string()
        }
    }
}
