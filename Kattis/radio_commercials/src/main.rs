use std::io::{stdin, Read};
use std::cmp::max;
use std::collections::VecDeque;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).expect("lecture de stdin");
    let mut lines = input.lines();
    let commercial_break: i32 = lines.next().expect("str").split_whitespace().skip(1).next().expect("str").parse().expect("i32");
    let mut vec: VecDeque<i32> = lines.next().expect("vec").split_whitespace().map(|e| e.parse::<i32>().expect("i32")).collect();
    vec = vec.iter().map(|e| e - commercial_break).collect();
    let mut result = Vec::new();
    if !vec.is_empty() {
        result.push(max(0, vec.pop_front().unwrap()));
    }
    for (index, i) in vec.iter().enumerate() {
        result.push(max(i + result[index], 0));
    }
    println!("{}", result.iter().max().expect("max"));
}
