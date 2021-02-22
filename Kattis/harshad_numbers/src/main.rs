use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut nb: u64 = input.lines().next().expect("str").parse().expect("u64");
    while !is_archad_number(nb) {
        nb += 1;
    }
    println!("{}",nb);
}

fn is_archad_number(nb: u64) -> bool {
    let somme: u64 = nb.to_string().chars().map(|e| e.to_string().parse::<u64>().expect("u8")).sum();
    return nb % somme == 0;
}