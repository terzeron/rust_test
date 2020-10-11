fn const_test1() {
    static LANGUAGE: &'static str = "Rust";
    const THRESHOLD: i32 = 10;

    fn is_big(n: i32) -> bool {
        n > THRESHOLD
    }

    println!("LANGUAGE={}", LANGUAGE);
    println!("THRESHOLD={}", THRESHOLD);
    println!("is_big(3)={}", is_big(3));
    println!("is_big(13)={}", is_big(13));
}

fn main() {
    println!("------ const_test1() ------");
    const_test1();
}
