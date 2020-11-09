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

fn declare_first_test() {
    let a_binding;
    {
        let x = 2;
        a_binding = x * x;
    }
    // 바인딩 선언 이후에 초기화를 하게 되면 block 바깥으로 영향이 발생함
    println!("a binding: {}", a_binding);

    let another_binding;
    //println!("another binding: {}", another_binding); // 에러, 초기화되지 않은 바인딩
    another_binding = 1;
    println!("another binding: {}", another_binding);
}

fn expression_test() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        x_cube + x_squared + x;
    };

    let z = {
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}

fn main() {
    println!("------ mutability_test() ------");
    mutability_test();
    println!("------ scope_shadowing_test() ------");
    scope_shadowing_test();
    println!("------ declare_first_test() ------");
    declare_first_test();
    println!("------ expression_test() ------");
    expression_test();
}
