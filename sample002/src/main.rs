fn main() {
    let n: u32 = 8;
    let fib1 = fib1(n);
    println!("fib with classic if:       fib1({}) = {}", n, fib1);
    println!("fib with default return:   fib2({}) = {}", n, fib2(n));
    println!("fib with pattern matching: fib3({}) = {}", n, fib3(n));
}

fn fib1(n: u32) -> u64 {
    if n < 2 {
        return 1;
    }

    return fib1(n - 1) + fib1(n - 2);
}

fn fib2(n: u32) -> u64 {
    if n < 2 {
        1
    } else {
        fib2(n - 1) + fib2(n - 2)
    }
}

fn fib3(n: u32) -> u64 {
    match n {
        0 | 1 => return 1,
        _ => fib3(n-1)+fib3(n-2)
    }
}