use std::io;
use std::io::Read;

fn compute_r2(r1:i32,s:i32)->i32
{
    2*s-r1
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut words = input.split_whitespace();
    let r1 = words.next().expect("r1").parse::<i32>().expect("Entier");
    let s = words.next().expect("s").parse::<i32>().expect("Entier");
    let r2=compute_r2(r1,s);
    println!("{}",r2);
}
