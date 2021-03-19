use std::io::{stdin, Read};
use std::cmp::max;
use std::collections::{VecDeque, HashSet, HashMap};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).expect("Lecture de stdin");
    input.lines().skip(1).for_each(|e| {
        let mut words = e.split_whitespace().map(|e| e.parse::<u64>().expect("u64"));
        let a = (words.next().unwrap()..=words.next().unwrap()).fold(0,|e,f| e + sum_of_digits(f));
        println!("{}",a);
    });
}

struct DigitIter(u64);

impl Iterator for DigitIter {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 { return None; }
        let ret = self.0 % 10;
        self.0 = self.0 / 10;
        Some(ret)
    }
}

fn sum_of_digits(entier: u64) -> u64 {
    return DigitIter(entier).sum::<u64>();
}

