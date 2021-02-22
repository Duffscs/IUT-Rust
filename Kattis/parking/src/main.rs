use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut words = input.split_whitespace().map(|e| e.parse::<i64>().expect("i64"));
    let nb_test_case = words.next().expect("i64");
    for _ in 0..nb_test_case {
        let nb_store = words.next().expect("i64");
        let min = words.clone().take(nb_store as usize).min().unwrap();
        let max = words.clone().take(nb_store as usize).max().unwrap();
        for _ in 0..nb_store {words.next();}
        println!("{}", (max - min) * 2);
    }
}