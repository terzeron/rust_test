fn structure_test1() {
    struct Nil;
    struct Pair(i32, f32);

    struct Point {
        x: f32,
        y: f32,
    }

    #[allow(dead_code)]
    struct Rectangle {
        p1: Point,
        p2: Point,
    }

    let point: Point = Point { x: 0.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);

    let Point { x: my_x, y: my_y } = point;
    let _rectangel = Rectangle {
        p1: Point { x: my_x, y: my_y },
        p2: point,
    };

    let _nil = Nil;

    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // 재구조화
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
}

fn enum_test1() {
    #![allow(dead_code)]
    enum Person {
        Engineer,
        Scientist,
        Height(i32),
        Weight(i32),
        Info {
            name: String,
            height: i32,
        },
    }

    fn inspect(p: Person) {
        match p {
            Person::Engineer => println!("is an engineer!"),
            Person::Scientist => println!("is a scientist!"),
            Person::Height(i) => println!("has a height of {}", i),
            Person::Weight(i) => println!("has a weight of {}", i),
            Person::Info { name, height } => { println!("{} is {} tall.", name, height); }
        }
    }

    let person = Person::Height(18);
    let amira = Person::Weight(10);
    let dave = Person::Info { name: "Dave".to_owned(), height: 72 };
    let rebecca = Person::Scientist;
    let rohan = Person::Engineer;
    inspect(person);
    inspect(amira);
    inspect(dave);
    inspect(rebecca);
    inspect(rohan);
}

fn enum_test2() {
    #![allow(dead_code)]
    enum Status {
        Rich,
        Poor,
    }
    enum Work {
        Civilian,
        Soldier,
    }

    use Status::{Poor, Rich};
    use Work::*;

    let status = Poor;
    let work = Civilian;
    match status {
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money!"),
    }
    match work {
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }
}

fn enum_test3() {
    use List::*;

    enum List {
        Cons(u32, Box<List>),
        Nil,
    }

    impl List {
        fn new() -> List {
            Nil
        }

        fn prepend(self, elem: u32) -> List {
            Cons(elem, Box::new(self))
        }

        fn len(&self) -> u32 {
            // List 타입인지 확인
            match *self {
                // self가 대여 중이라서 tail의 참조를 얻은 다음에 길이를 increment
                Cons(_, ref tail) => 1 + tail.len(),
                Nil => 0
            }
        }

        fn stringify(&self) -> String {
            match *self {
                Cons(head, ref tail) => {
                    format!("{}, {}", head, tail.stringify())
                }
                Nil => {
                    format!("Nil")
                }
            }
        }
    }
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}

fn const_test1() {
    static LANGUAGE: &'static str = "Rust";
    const THRESHOLD: i32 = 10;

    fn is_big(n: i32) -> bool {
        n > THRESHOLD
    }

    println!("LANGUAGE={}", LANGUAGE);
    println!("THRESHOLD={}", THRESHOLD);
    println!("is_big(3)={}", is_big(3));
    println!("is_big(13)={}", is_big(13));
}

fn main() {
    println!("--------- structure_test1() ---------");
    structure_test1();
    println!("--------- enum_test1 ---------");
    enum_test1();
    println!("--------- enum_test2 ---------");
    enum_test2();
    println!("--------- enum_test3 ---------");
    enum_test3();
    println!("------ const_test1() ------");
    const_test1();
}
