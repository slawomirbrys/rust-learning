use std::io::stdin;

fn main() {
    let mut buf = String::new();
    let val = match stdin().read_line(&mut buf) {
        Ok(size) => size,
        Err(_e) => 0
    };

    if val == 0 {
        return;
    }

    let val: u32 = buf.trim().parse::<u32>().unwrap();
    println!("is_even_if returns({}) = {}", val, is_even_if(val));
    println!("is_even_match returns({}) = {}", val, is_even_if(val));
    println!("sum_all_in_loop({}) = {}", val, sum_all_in_loop(val));
}


fn is_even_if(val: u32) -> bool {
    let res = if val % 2 == 0 {
        true
    } else {
        false
    };
    res
}

fn is_even_match(val: u32) -> bool {
    let res = match val % 2 {
        0 => true,
        _ => false
    };
    res
}

fn sum_all_in_loop(val: u32) -> u32 {
    let mut number = 1;
    let mut sum = 0;
    let sum = loop {
        if number > val {
            break sum;
        }
        sum = sum + number;
        number = number + 1;
    };
    sum
}