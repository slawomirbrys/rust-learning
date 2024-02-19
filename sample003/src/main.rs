fn main() {
    main1();
    main2();
    main3();
}

fn main1() {
    let s = String::from("Hello Rust by value");
    print_by_val(s);

    // Compilation error: 's' is destroyed in first `print()` call
    //print(s);
}

fn main2() {
    let s = String::from("Hello Rust by reference");
    print_by_ref(&s);
    print_by_ref(&s);
    println!("Final by reference: {}", s);
}

fn main3() {
    let mut s = String::from("Hello Rust by reference");
    print_by_mutable_ref(&mut s);
    print_by_mutable_ref(&mut s);
    println!("Final by mutable reference: {}", s);
}

fn print_by_val(s: String) {
    println!("{}", s);
}

fn print_by_ref(s: &String) {
    println!("{}", s);
}

fn print_by_mutable_ref(s: &mut String) {
    println!("{}", s);
    s.push('!');
}