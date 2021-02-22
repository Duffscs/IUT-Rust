use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let nb: i32 = input.lines().next().expect("str").parse().expect("i64");
    let res: i64 = format!("{:b}", nb).chars()
        .enumerate()
        .map(|e| {
            return e.1.to_string().parse::<i64>().unwrap() << e.0;
        }).sum();
    println!("{}", res);
}