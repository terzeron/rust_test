use std::io;

fn main() {
    // floating point number
    let x = 2.0;
    let y: f32 = 3.0;
    println!("x is {}", x);
    println!("y is {}", y);

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;
    println!("sum is {}", sum);
    println!("difference is {}", difference);
    println!("product is {}", product);
    println!("quotient is {}", quotient);
    println!("truncated is {}", truncated);
    println!("remainder is {}", remainder);

    // boolean
    let t = true;
    let f: bool = false;
    println!("t is {}", t);
    println!("f is {}", f);

    // char
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("c is {}", c);
    println!("z is {}", z);
    println!("heart_eyed_cat is {}", heart_eyed_cat);

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup is {:?}", tup);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("five_hundred is {five_hundred}");
    println!("six_point_four is {six_point_four}");
    println!("one is {}", one);

    // array
    let a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("a is {:?}", a);
    println!("months is {:?}", months);
    let b = [3; 5];
    println!("b is {:?}", b);
    println!("first element of a is {}", a[0]);
    println!("second element of a is {}", a[1]);
    println!("Please input an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}
