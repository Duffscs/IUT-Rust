use std::io;
use std::io::Read;

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut word = input.split_whitespace().map(|e| e.parse::<f64>().expect("f64"));
    let point1 = Point { x: word.next().expect("d"), y: word.next().expect("d") };
    let point2 = Point { x: word.next().expect("d"), y: word.next().expect("d") };
    let point3 = Point { x: word.next().expect("d"), y: word.next().expect("d") };
    let mut point4 = Point { x: point2.x, y: point2.y };
    if point1.x == point2.x {
        point4.x = point3.x;
    } else if point2.x == point3.x {
        point4.x = point1.x;
    }

    if point1.y == point2.y {
        point4.y = point3.y;
    } else if point2.y == point3.y {
        point4.y = point1.y;
    }

    println!("{} {}", point4.x, point4.y);
}