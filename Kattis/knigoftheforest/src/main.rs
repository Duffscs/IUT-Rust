use std::io;
use std::io::Read;
use std::collections::{VecDeque, HashMap, HashSet, BinaryHeap};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut word = input.split_ascii_whitespace();

    word.next();
    let n: i32 = word.next().expect("read").parse().expect("i32");
    let mut participant: BinaryHeap<(i32, i32)> = BinaryHeap::new();
    let mut attente: Vec<(i32, i32)> = Vec::new();
    let karl = word.map(|e| e.parse::<i32>().expect("i32"))
        .collect::<Vec<i32>>()
        .chunks(2)
        .map(|e| {
            if e[0] == 2011 {
                participant.push((e[1], e[0]));
            } else {
                attente.push((e[1], e[0]));
            }
            (e[1], e[0])
        }).collect::<Vec<(i32,i32)>>()[0];


    attente.sort_by_key(|e| -e.1);

    for year in 0..n {
        if year != 0 {
            participant.push(attente.pop().unwrap());
        }
        let max = participant.pop().unwrap();

        if max == karl {
            println!("{}", year + 2011);
            return;
        }
    }
    println!("unknown");
}