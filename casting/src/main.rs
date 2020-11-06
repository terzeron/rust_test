#![allow(overflowing_literals)]

fn casting_test() {
    let decimal = 65.4321_f32;
    //let integer: u8 = decimal;

    let integer = decimal as u8;
    let character = integer as char;
    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    println!("1000 as a u16 is : {}", 1000 as u16);
    println!("1000 as a u8 is : {}", 1000 as u8);
    println!("-1 as a u8 is : {}", (-1i8) as u8);
    println!("1000 mod 256 is : {}", 1000 % 256);

    println!("128 as a i16 is : {}", 128 as i16);
    println!("128 as a i8 is : {}", 128 as i8);
    println!("1000 as a i8 is : {}", 1000 as i8);
    println!("232 as a i8 is : {}", 232 as i8);
}

fn literal_test() {
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    let i = 1;
    let f = 1.0;

    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}

fn inference_test() {
    let elem = 5u8;
    let mut vec = Vec::new(); // Vec<_> 였다가 Vec<u8>로 결정됨
    vec.push(elem);
    println!("{:?}", vec);
}

fn alias_test() {
    type NanoSecond = u64;
    type Inch = u64;

    #[allow(non_camel_case_types)]
    type u64_t = u64;

    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;
    println!("{} nanoseconds + {} inches = {} unit?", nanoseconds, inches, nanoseconds + inches);
}

fn main() {
    println!("------ casting_test ------");
    casting_test();
    println!("------ literal_test ------");
    literal_test();
    println!("------ inference_test ------");
    inference_test();
    println!("------ alias_test ------");
    alias_test();
}
