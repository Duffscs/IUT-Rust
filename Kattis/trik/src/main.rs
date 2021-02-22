use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut vec = vec![true, false, false];
    input.chars().for_each(|e| {
        if e == 'A' {
            vec.swap(0, 1);
        } else if e == 'B' {
            vec.swap(1, 2);
        } else if e == 'C' {
            vec.swap(2, 0);
        }
    });
    for i in 0..vec.len() {
        if vec[i] {
            println!("{}", i + 1);
        }
    }
}