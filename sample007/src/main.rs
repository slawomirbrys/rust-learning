fn main() {
    let mut color: Color = Color::Red;
    println!("My color is {}", color_to_str(&color));

    color = Color::Green;
    println!("My color is {}", color_to_str(&color));

    // Can have uninitialized enum, but can't use it, compilation error
    let _c: Color;
    //println!("Default enum value is {}", color_to_str(&c));


    let mut color_ex: ColorEx = ColorEx::RGB(0.5, 0.1, 0.2);
    println!("My color is {}", color_ex_to_str(&color_ex));

    color_ex = ColorEx::Pantone("pink".to_string());
    println!("My color is {}", color_ex_to_str(&color_ex));
}

enum Color {
    Red,
    Green,
    Blue,
}

fn color_to_str(color: &Color) -> String {
    match color {
        Color::Red => "red".to_string(),
        Color::Blue => "blue".to_string(),
        _ => "unknown".to_string()
    }
}

enum ColorEx {
    RGB(f64, f64, f64),
    HSV(f64, f64, f64),
    Pantone(String),
}

fn color_ex_to_str(color: &ColorEx) -> String {
    match color {
        ColorEx::RGB(r, g, b) => format!("rgb({}, {}, {})", r, g, b),
        ColorEx::HSV(h, s, v)=> format!("hsv({}, {}, {})", h, s, v),
        ColorEx::Pantone(name) => format!("pantone({})", name)
    }
}

#[allow(dead_code)]
enum ColorExNamed {
    RGB { red: f64, green: f64, blue: f64 },
    HSV { hue: f64, saturation: f64, value: f64},
    Pantone { name: String }
}
