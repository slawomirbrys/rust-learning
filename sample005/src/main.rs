fn main() {
    main_by_ref();
    main_by_val();
}


fn main_by_ref() {
    let s = String::from("Test string");
    let s_sub = substr_ref(&s, 0, 4);

    println!("Original string: {}", s);
    println!("Substring by ref: {}", s_sub);
}

fn main_by_val() {
    let s = String::from("Test string");
    let s_sub = substr_val(&s, 0, 4);

    println!("Original string: {}", s);
    println!("Substring by val: {}", s_sub);
}

fn substr_ref(s: &str, pos: usize, len: usize) -> &str {
    &s[pos..pos+len]
}

fn substr_val(s: &String, pos: usize, len: usize) -> String {
    let sub = &s[pos..pos+len];
    return String::from(sub);
}