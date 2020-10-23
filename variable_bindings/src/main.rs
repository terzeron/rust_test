fn mutability_test() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);
    mutable_binding += 1;
    println!("After mutation: {}", mutable_binding);

    //_immutable_binding + 1; // error
}
fn main() {
    println!("------ mutability_test() ------");
    mutability_test();
}
