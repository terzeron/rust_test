fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn main() {
    let mut s = String::from("hello");
    //let mut s = "hello";
    s.push_str(", world!");
    println!("{}", s);

    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // move
    let s1 = String::from("hello");
    let s2 = s1; // move
    println!("s2 = {}", s2);

    // clone
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4);

    let s = String::from("hello");
    takes_ownership(s);
    //println!("{}", s); // error: value borrowed here after move
    let x = 5;
    makes_copy(x);
    println!("{}", x);

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("s1 = {}, s3 = {}", s1, s3);
    
    // 소유권이 calculate_length()로 넘어갔다가 되돌아오는 번잡한 방식
    let s4 = String::from("hello");
    let (s2, len) = calculate_length(s4);
    println!("The length of '{}' is {}", s2, len);
}
