use std::fmt::{self, Formatter, Display};

fn print_test() {
    println!("Hello, world!");

    let x = 5 + 5;
    println!("Is 'x' 10 or 100? x = {}", x);

    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("{subject} {verb} {object}", object="the lazy dog", subject="the quick brown fox",
             verb="jumps over");

    println!("{number:>width$}", number=1, width=6);
    println!("{number:>0width$}", number=1, width=6);

    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    println!("My name is {0}, {1} {0}", "Bond", "James");

    #[allow(dead_code)]
    struct Structure(i32);
    //println!("This struct '{}' won't print...", Structure(3));

    let pi = 3.141592;
    println!("Pi is roughly {:.*}", 3, pi);
}


fn fmt_test1() {
    // {:?}는 format for debugging
    println!("{:?} months in a year.", 12);
    // debugging 모드에서는 따옴표가 붙는다는 점에 유의할 것
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
    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let List(ref vec) = *self;
            // try!는 예외발생을 감지하는 매크로
            // 예외가 발생하면 에러를 던지고 아니면 진행함
            write!(f, "[")?;
            for (count, v) in vec.iter().enumerate() {
                if count != 0 { write!(f, ", ")?; }
                write!(f, "{}: {}", count, v)?;
            }
            write!(f, "]")
        }
    }

    let v = List(vec![50, 37, 99, 78]);
    println!("{}", v);
}


fn fmt_test4() {
    struct City {
        name: &'static str,
        lat: f32,
        lon: f32,
    }

    impl Display for City {
        // f: buffer
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
            let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

            write!(f, "{}: {:.3}'{} {:.3}'{}", self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
        }
    }

    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }

    #[derive(Debug)]
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }

    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{:?}", *color);
    }
}

fn fmt_test5() {
    #[derive(Debug)]
    struct MinMax(i64, i64);

    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    let minmax = MinMax(0, 14);
    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3,3 );
    println!("{big}, {small}", small = small_range, big = big_range);
}

fn main() {
    print_test();
    fmt_test1();
    fmt_test2()
    fmt_test3();
    fmt_test4();
    fmt_test5();
}
