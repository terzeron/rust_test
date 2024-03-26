fn guard_test() {
    let pair = (2, -2);
    println!("Tell me about {:?}", pair);

    // guard는 match의 필터로서 기능함
    match pair {
        (x, y) if x == y => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}

fn main() {
    println!("\n------ guard_test() ------");
    guard_test();
}
