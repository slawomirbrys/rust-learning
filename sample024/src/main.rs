use std::fmt;
use std::fmt::{Formatter};

#[derive(Debug)]
struct PointDebug(u32, u32);

#[derive(Debug)]
struct PointDisplay(u32, u32);

impl fmt::Display for PointDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

fn main() {
    let p1 = PointDebug(10, 12);
    println!("{:?}", p1);

    let p2 = PointDisplay(10, 20);
    println!("{:?}", p2);
    println!("{}", p2);
}
