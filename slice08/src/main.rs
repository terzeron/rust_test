fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn primitive_substring_test() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("length of '{s}'={word}");
    s.clear();
}

fn slice_test1() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("hello={hello}, world={world}");
}

fn slice_test2() {
    let s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..];
    println!("hello={hello}, world={world}");
}

fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn first_word_by_slice_test() {
    let s = String::from("hello world");
    let word = first_word2(&s);
    println!("first word={word}");
}

fn array_slice_test() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("slice={:?}", slice);
    assert_eq!(slice, &[2, 3]);
}

fn main() {
    primitive_substring_test();

    slice_test1();
    slice_test2();
    
    first_word_by_slice_test();
    
    array_slice_test();
}
