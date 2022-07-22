use std::io::{self, BufRead, Read};
use structopt::StructOpt;
use std::collections::VecDeque;
use std::ops::Add;

#[derive(StructOpt)]
struct Options {
    #[structopt()]
    stop: usize
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let nb_car =input.chars().count();
    let nb_mot =input.split_whitespace().count();
    let nb_ligne =input.lines().count();
    println!("Lignes : {}", nb_ligne);
    println!("Mots : {}", nb_mot);
    println!("Caractères : {}", nb_car);
}
