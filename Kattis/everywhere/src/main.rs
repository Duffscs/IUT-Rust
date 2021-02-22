use std::io;
use std::io::Read;
use std::collections::{HashSet};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut words = input.split_whitespace();
    let nb_test_case = words.next().expect("n").parse::<i32>().expect("i32");

    for _ in 0..nb_test_case {
        let nb_flight = words.next().expect("n").parse::<usize>().expect("usize");
        let nb  = words.clone().take(nb_flight).collect::<HashSet<&str>>().len();
        for _ in 0..nb_flight { words.next(); }
        //words = words.skip(nb_flight as usize);
        print!("{} ",nb);
    }
}
