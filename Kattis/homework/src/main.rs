use std::io;
use std::io::Read;


fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let count : i64 = input.lines().next().expect("str1").split(";")
        .map(|e| {
        let mut words = e.split("-");
        let nb1 : i64 = words.next().expect("str").parse().expect("i64");
        return if let Some(nb2) = words.next() {
            nb2.parse::<i64>().unwrap() - nb1 + 1
        } else {
            1
        }
    }).sum();
    println!("{}",count);

}
