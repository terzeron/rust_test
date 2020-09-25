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
    println!("mutable: {}", mutable);
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

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn tuple_test() {
    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true);
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("the reversed pair is {:?}", reverse(pair));

    println!("one element tuple: {:?}", (5u32, ));
    println!("just an integer: {:?}", (5u32));

    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
}

fn array_test() {
    use std::mem;

    fn analyze_slice(slice: &[i32]) {
        println!("first element of the slice: {}", slice[0]);
        println!("the slice has {} elements", slice.len());
    }

    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [53; 500];

    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    println!("array size: {}", xs.len());
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[2..10]);

    println!("{}", xs[999]);
}

fn main() {
    println!("\n------ type_test() ------");
    type_test();
    println!("\n------ arithmetic_test() ------");
    arithmetic_test();
    println!("\n------ tuple_test() ------");
    tuple_test();
    println!("\n------ array_test() ------");
    array_test();
}
