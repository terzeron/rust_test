fn match_test() {
    let number = 13;
    println!("Tell me about {}", number);
    match number {
        // 단순 리터럴 매칭
        1 => println!("One!"),
        // or로 묶어서 리터럴 매칭
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // 범위 매칭
        13...19 => println!("A teen"),
        // 디폴트
        _ => println!("Ain't special"),
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("{} -> {}", boolean, binary)
}

fn main() {
    println!("\n------ match_test() ------");
    match_test();
}
