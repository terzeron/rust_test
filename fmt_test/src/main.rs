fn fmt_test1() {
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.", "Slater", "Christian", actor = "actor is");

    // derive 속성으로 아래 struct를 fmt::Debug의 구현을 제공함
    // Java로 치면 Object.toString()
    #[derive(Debug)]
    struct Structure(i32);
    #[derive(Debug)]
    struct Deep(Structure);
    println!("Now {:?} will print!", Structure(3));
    //println!("Now {} will print!", Structure(3));
    println!("Now {:?} will print!", Deep(Structure(7)));
}


fn fmt_test2() {
    use std::fmt;

    // Java로 치면 toString() 재정의
    struct Structure(i32);
    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    println!("Now {} will print!", Structure(3));

    #[derive(Debug)]
    struct Point2 {
        x: f64,
        y: f64
    }
    impl fmt::Display for Point2 {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
    let point = Point2{x: 3.3, y: 4.1};
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
}

fn fmt_test3() {
    use std::fmt;

    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let List(ref vec) = *self;
            // try!는 예외발생을 감지하는 매크로
            // 예외가 발생하면 에러를 던지고 아니면 진행함
            try!(write!(f, "["));
            for (count, v) in vec.iter().enumerate() {
                if count != 0 { try!(write!(f, ", ")); }
                try!(write!(f, "{}: {}", count, v));
            }
            write!(f, "]")
        }
    }

    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}



fn main() {
    fmt_test1();
    fmt_test2();
    fmt_test3();
}
