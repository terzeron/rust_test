mod test;

fn main() {
    let x: i32 = 5;
    let _y: i32; // unused variable
    assert_eq!(x, 5);
    println!("Success!");

    let mut x1 = 1;
    x1 += 2;
    assert_eq!(x1, 3);
    println!("Success!");

    let x2: i32 = 10;
    {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x2, y);
    }
}
