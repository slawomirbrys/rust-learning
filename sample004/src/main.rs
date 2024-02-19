fn main() {
    mut_binding();
    rebinding();
    rebinding_mine();
}


fn mut_binding() {
    println!("MUT BINDING");
    let mut i = 1;
    println!("initially, i = {}", i);
    {
        i = i + 1;
        println!("inside block, i = {}", i);
    }
    println!("outside block, i = {}", i);
}

fn rebinding() {
    println!("REBINDING");
    let i = 1;
    println!("initially, i = {}", i);
    {
        let i = i + 1;
        println!("inside block, i = {}", i);
    }
    println!("outside block, i = {}", i);
    let i = i + 1;
    println!("after rebinding, i = {}", i);
}

fn rebinding_mine() {
    println!("REBINDING MINE");
    let i = {
        let mut i = 1;
        println!("inside block, i = {}", i);
        i = i + 1;
        i
    };
    println!("after rebinding, i = {}", i);
}