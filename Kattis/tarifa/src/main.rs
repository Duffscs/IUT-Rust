use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut words = input.split_whitespace();
    let data_number : i32 = words.next().expect("str").parse().expect("i32");
    let mut count = data_number;
    words.skip(1).for_each(|e| {
        count += data_number - e.parse::<i32>().expect("i32");
    });
    println!("{}", count);


}