fn match_destructing_to_tuple_test() {
    let pair = (0, -2);
    println!("Tell me about {:?}", pair);
    match pair {
        // tuple의 역구조화
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
    // enum의 역구조화
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
    // 4라는 값을 가지는 메모리 공간에 대한 레퍼런스를 할당함
    let reference = &4;
    println!("reference={:?}", reference);
    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }
    // *을 사용하면 역참조
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }
    // 위 두가지 매칭은 동일한데 문법적으로만 약간 다름

    let _not_a_reference = 3;
    // let _is_a_reference = &3; 과 동일함
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

fn match_structs_test() {
    // struct 타입 정의
    struct Foo { x: (u32, u32), y: u32 }
    ;
    // 인스턴스
    let foo = Foo { x: (1, 2), y: 3 };
    // 역구조화를 위한 실제 변수 바인딩
    let Foo { x: (a, b), y } = foo;
    println!("a={}, b={}, y={}", a, b, y);

    let Foo { y: i, x: j } = foo;
    println!("i={:?}, j={:?}", i, j);

    let Foo { y, .. } = foo;
    println!("y={}", y);
}

fn main() {
    println!("\n------ match_destructing_to_tuple_test() ------");
    match_destructing_to_tuple_test();
    println!("\n------ match_enum_test() ------");
    match_enum_test();
    println!("\n------ match_pointers_ref_test() ------");
    match_pointers_ref_test();
    println!("\n------ match_structs_test() ------");
    match_structs_test();
}
