fn main() {
    let s = "This unlucky string";
    let s_sub = substr_ref(&s, 7, 12);
    let s_sub_lifetime = substr_ref_lifetime(&s, 7, 12);

    println!("Original: {}", s);
    println!("Substring: {}", s_sub);
    println!("Substring with lifetime: {}", s_sub_lifetime);
}

fn substr_ref(s: &str, pos: usize, len: usize) -> &str {
    &s[pos..pos+len]
}

fn substr_ref_lifetime<'a>(s: &'a str, pos: usize, len: usize) -> &'a str {
    &s[pos..pos+len]
}
