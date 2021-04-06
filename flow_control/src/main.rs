#![allow(unreachable_code)]

fn if_else_test() {
    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");
        10 * n
    } else {
        println!(", and is a big number, halve the number");
        n / 2
    };

    println!("{} -> {}", n, big_n);
}

fn loop_test() {
    let mut count = 0u32;
    println!("Let's count until infinity!");

    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }
        println!("{}", count);
        if count == 5 {
            println!("OK, that's enough");
            break;
        }
    }
}

fn nesting_and_labels_test() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'outer;
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");
}

fn returning_from_loops_test() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("{}", result);
    assert_eq!(result, 20);
}

fn while_test() {
    let mut n = 1;

    while n < 15 {
        if n % 6 == 0 {
            print!("fizzbuzz ");
        } else if n % 2 == 0 {
            print!("fizz ");
        } else if n % 3 == 0 {
            print!("buzz ");
        } else {
            print!("{} ", n);
        }

        n += 1;
    }
    println!();
}

fn for_and_range_test() {
    for n in 1..15 {
        if n % 6 == 0 {
            print!("fizzbuzz ");
        } else if n % 2 == 0 {
            print!("fizz ");
        } else if n % 3 == 0 {
            print!("buzz ");
        } else {
            print!("{} ", n)
        }
    }
    println!("")
}

fn match_test() {
    let number = 13;
    println!("Tell me about {}", number);
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13...19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("{} -> {}", boolean, binary)
}

fn match_destructing_to_tuple_test() {
    let pair = (0, -2);
    println!("Tell me about {:?}", pair);
    match pair {
        (0, y) => println!("First is 0 and y is {:?}", y),
        (x, 0) => println!("x is {:?} and last is 0", x),
        _ => println!("It doesn't matter what they are"),
    }
}

#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn match_enum_test() {
    let color = Color::RGB(122, 17, 40);
    println!("what color is it?");
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) => println!("Red: {}, green: {}, and blue {}!", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, and value {}!", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, and lightness {}!", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, and yellow {}!", c, m, y),
        Color::CMYK(c, m, y, k) => println!("Cyan: {}, magenta: {}, yellow: {}, and key(black) {}!", c, m, y, k),
    }
}

fn match_pointers_ref_test() {
    // &, ref, ref mut를 사용하면 역구조화
    let reference = &4;
    println!("reference={:?}", reference);
    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }
    // *을 사용하면 역참조
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    let _not_a_reference = 3;
    let ref _is_a_reference = 3;
    let value = 5;
    let mut mut_value = 6;
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }
    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        }
    }
}

fn main() {
    println!("\n------ if_else_test() ------");
    if_else_test();
    println!("\n------ loop_test() ------");
    loop_test();
    println!("\n------ nesting_and_labels_test() ------");
    nesting_and_labels_test();
    println!("\n------ returning_from_loops_test() ------");
    returning_from_loops_test();
    println!("\n------ while_test() ------");
    while_test();
    println!("\n------ for_and_range_test() ------");
    for_and_range_test();
    println!("\n------ match_test() ------");
    match_test();
    println!("\n------ match_destructing_to_tuple_test() ------");
    match_destructing_to_tuple_test();
    println!("\n------ match_enum_test() ------");
    match_enum_test();
    println!("\n------ match_pointers_ref_test() ------");
    match_pointers_ref_test();
}
