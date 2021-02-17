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

    let big_n =
        if n < 10 && n > -10 {
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

    assert_eq!(result, 20);
}

fn while_test() {
    let mut n = 1;

    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        n += 1;
    }
}

fn for_and_range_test() {
    for n in 1..101 {
        if n % 15 == 0 {
            print!("fizzbuzz ");
        } else if n % 3 == 0 {
            print!("fizz ");
        } else if n % 5 == 0 {
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
        _ => println!("Ain't special")
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("{} -> {}", boolean, binary)
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
}
