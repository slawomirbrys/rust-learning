fn main() {
    println!("Hello, world!");

    println!("{} days", 31);
    println!("{} text", "test");
    println!("{} char", 'a');

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    let email = "test@test.test";
    println!("My email is {email}");

    println!("Base 10:               {}",   69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
    println!("Base 16 (hexadecimal): {:X}", 69420); // 10F2C
    println!("Base 255 (hex):        {:p}", email);   // ff

    println!("{number:>5}", number=1);
    println!("{number:0>5}", number=1); // 00001
    println!("{number:0<5}", number=1); // 10000
    println!("{number:*<5}", number=1); // 1****

    println!("{number:0>width$}", number=1, width=5);

    println!("One million is written as {}", 1_000_000);
    println!("One million is written as {}", 1_000_000u32);

    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    eprintln!("Some error occurred");
}
