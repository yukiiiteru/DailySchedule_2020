#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

struct Nil;

struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle { p1: p1, p2: p2} = rect;
    ((p2.y - p1.y) * (p2.x - p1.x)).abs()
}

fn square(point: Point, size: f32) -> Rectangle {
    Rectangle {
        p1: Point { x: point.x, y: point.y },
        p2: Point { x: point.x + size, y: point.y + size },
    }
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    println!("{:?}", peter);

    let point: Point = Point { x: 0.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);

    let new_point = Point { x: 0.1, ..point };
    println!("second point: ({}, {})", new_point.x, new_point.y);

    let Point { x: my_x, y: my_y } = point;
    let _rectangle = Rectangle {
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    let _nil = Nil;
    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("the area of rectangle is {}", rect_area(_rectangle));
    let point = Point { x: 0.1, y: 0.1 };
    println!("this is a square {:?}", square(point, 2.0));
}
