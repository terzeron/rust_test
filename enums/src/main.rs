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

fn main() {
    println!("--------- enum_test1 ---------");
    enum_test1();
    println!("--------- enum_test2 ---------");
    enum_test2();
}
