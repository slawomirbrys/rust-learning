fn main() {
    let i = 10;
    print(i);
    // ownership of 'i' not taken for simple types (?)
    print(i);
}

fn print(i: u32) {
    println!("{}", i);
}
