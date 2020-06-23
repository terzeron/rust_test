fn type_test() {
    let logical: bool = true;
    println!("bool: {}", logical);
    let a_float: f64 = 1.01;
    println!("f64: {}", a_float);
    let an_integer = 5i32;
    println!("i32: {}", an_integer);

    let default_float = 3.5;
    println!("implicit f64: {}", default_float);
    let default_integer = 7;
    println!("implicit i32: {}", default_integer);

    let mut mutable = 12;
    //mutable = true; // error
    mutable = 33;
    println!("mutable: {}", mutable);
}


fn arithmetic_test() {
    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 + 2 = {}", 1 + 2);
    println!("1 - 2 = {}", 1i32 - 2);
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
    println!("One million is written as {}", 1_000_000u32);
}


fn main() {
    type_test();
    arithmetic_test();
}
