use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    input.lines().skip(1)
        .for_each(|e| {
            let mut words = e.split_whitespace();
            let case = words.next().expect("str");
            let nb_day = words.next().expect("str").parse::<i64>().expect("i64") + 1;
            let sum = (0..=nb_day).sum::<i64>() - 1;
            println!("{} {}", case, sum );
        });
}