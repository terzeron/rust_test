fn structure_test() {
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

fn main() {
    println!("--------- structure test ---------");
    structure_test()
}
