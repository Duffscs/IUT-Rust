use std::io;
use std::io::Read;
use std::collections::VecDeque;
use std::process;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut index = 0;
    let mut pile: Vec<char> = Vec::new();
    input.lines().skip(1).next().expect("str")
        .chars().for_each(|e| {
        if is_opener(e) {
            pile.push(get_closure(e))
        }
        if is_closure(e) {
            if let Some(element) = pile.pop() {
                if element != e {
                    println!("{} {}", e, index);
                    process::exit(0);
                }
            } else {
                println!("{} {}", e, index);
                process::exit(0);
            }
        }
        index += 1;
    });
    println!("ok so far");
}

fn get_closure(car: char) -> char {
    if car == '(' { return ')'; }
    if car == '[' { return ']'; }
    if car == '{' { return '}'; }
    return ' ';
}

fn is_opener(car: char) -> bool {
    car == '{' || car == '[' || car == '('
}

fn is_closure(car: char) -> bool {
    car == ')' || car == '}' || car == ']'
}