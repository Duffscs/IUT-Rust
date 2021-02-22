use std::io;
use std::io::Read;
use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut words = input.split_whitespace().map(|e| e.parse::<i64>().expect("i64"));
    let mut nb = words.next().expect("i64");
    while nb != -1 {
        let mut prev = 0;
        let mut distance = 0;
        words.clone().take((nb * 2) as usize)
            .collect::<Vec<i64>>()
            .chunks(2)
            .for_each(|e| {
                distance += e[0] * (e[1] - prev);
                prev = e[1];
            });
        println!("{} miles", distance);
        for _ in 0..(nb*2) { words.next(); }
        nb = words.next().expect("i64");
    }
}