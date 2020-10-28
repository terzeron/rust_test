fn mutability_test() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);
    mutable_binding += 1;
    println!("After mutation: {}", mutable_binding);

    //_immutable_binding + 1; // error
}

fn scope_shadowing_test() {
    let long_lived_binding = 1;

    {
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);
        let long_lived_binding = 5_f32;
        println!("inner long: {}", long_lived_binding);
    }

    //println!("outer short: {}", short_lived_binding); // error
    println!("outer long: {}", long_lived_binding);

    let long_lived_binding = 'a';
    println!("outer long: {}", long_lived_binding);
}

fn main() {
    println!("------ mutability_test() ------");
    mutability_test();
    println!("------ scope_shadowing_test() ------");
    scope_shadowing_test();
}
