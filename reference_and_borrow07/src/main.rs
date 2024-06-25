fn calculate_length(s: &String) -> usize {
    s.len()
}

fn borrow_test() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // s1의 reference를 immutable로 전달해서 참조만 할 수 있도록 함
    println!("The length of '{}' is {}.", s1, len);
}

fn change1(_some_string: &String) {
    //_some_string.push_str(", world"); // error, immutable하므로 변경 불가
}

fn immutable_reference_change_test() {
    let s = String::from("hello");
    change1(&s);
    println!("s: {s}")
}

fn change2(some_string: &mut String) { // 받은 쪽에서도 mutable reference로 선언
    some_string.push_str(", world");
}

fn mutable_reference_change_test() {
    let mut s = String::from("hello");
    change2(&mut s); // mutable reference라고 선언하면서 전달
    println!("s: {s}")
}

fn sequential_mutable_reference_test() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    //let r2 = &mut s; // mutable reference는 연속으로 borrow할 수 없음
    println!("{}", r1);
}

fn mutable_after_immutable_test1() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    //let r3 = &mut s; // error, immutable reference를 만든 다음에 mutable reference를 만들고 그 immutable을 사용하면 갑자기 원하지 않는 변경이 발생할 수 있어서 문제가 됨
    println!("{}, {}", r1, r2);
}

fn mutable_after_immutable_test2() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s; // 먼저 선언된 immutable reference가 사용되지 않는다면 mutable reference를 사용할 수 있음
}

fn dangle() -> String {
    // -> String 대신에 -> &String으로 하면 dangling reference가 됨
    let s = String::from("hello");
    s
    //s 대신에 &s로 하면 dangling reference가 됨
}

fn dangling_reference_test() {
    let reference_to_nothing = dangle();
}

fn main() {
    borrow_test();

    immutable_reference_change_test();
    
    mutable_reference_change_test();

    sequential_mutable_reference_test();

    mutable_after_immutable_test1();
    mutable_after_immutable_test2();
    
    dangling_reference_test();
}
