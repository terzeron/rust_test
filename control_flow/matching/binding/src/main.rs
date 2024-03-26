fn binding_test() {
    fn age() -> u32 {
        15
    }

    println!("Tell me type of person you are");
    match age() {
        0 => println!("I'm not born yet I guess"),
        // @를 이용하여 값을 변수에 바인딩함
        n @ 1...12 => println!("I'm a child of age {:?}", n),
        n @ 13...19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm an old person of age {:?}", n),
    }
}

fn main() {
    println!("\n------ binding_test() ------");
    binding_test();
}
