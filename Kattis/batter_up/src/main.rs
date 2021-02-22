use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut count = 0;
    let avg : f64= input.split_whitespace().skip(1).map(|e| {
        let nb: i64 = e.parse().expect("i64");
        return if nb != -1 {
            count += 1;
            nb
        } else {
            0
        };
    }).sum::<i64>() as f64 / count as f64 ;
    println!("{}", avg);
}