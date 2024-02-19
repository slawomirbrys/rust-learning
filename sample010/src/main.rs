fn main() {
    let s = String::from("Hello world");
    let s_substr= substr(&s, 0, 5);

    let s_clone = s_substr.to_string();

    println!("{}", s);
    println!("{}", s_substr);

    // here 's' ends its lifetime
    consume(s);

    // this will not compile, ownership taken by consume
    //println!("{}", s);
    // this will not compile, it's based on borrowed 's'
    //println!("{}", s_substr);

    println!("{}", s_clone);

}

fn substr(s: &str, pos: usize, len: usize) -> &str {
    &s[pos..pos + len]
}

fn consume(_: String) {}

