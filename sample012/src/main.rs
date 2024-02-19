static HELLO: &str = "Hi";

fn main() {

    let hello: &'static str = "Hello";
    let mut hello = "Good morning";
    hello = "Goodbye";

    println!("{}, world!", hello);

    // real static cannot be changed
    //let mut HELLO = "a";
    println!("{}, world!", HELLO);
}


