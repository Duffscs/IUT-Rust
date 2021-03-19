use std::io::{stdin, Read};
use std::cmp::max;
use std::collections::VecDeque;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(& mut input).expect("Lecture de stdin");
    input.lines().for_each(|line| {
        let mut vec : Vec<i32> = line.split_whitespace().map(|e| e.parse::<i32>().expect("i32")).collect();
        let somme = somme_element_de_proche_en_proche(vec);
        println!("{:?}",somme);
        println!("{}", plus_grande_somme_consecutive(somme));
    });
}

fn somme_element_de_proche_en_proche(vec: Vec<i32>) -> Vec<i32> {
    return vec.windows(2).map(|i| i[0] - i[1]).collect();
}

fn plus_grande_somme_consecutive(vec: Vec<i32>) -> i32 {
    let mut temp = 0;
    return
        vec.iter().map(|e| {
            temp = max(e + temp, 0);
            return temp;
        }).fold(0, max);
}