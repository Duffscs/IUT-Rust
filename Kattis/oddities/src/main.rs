use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut words = input.split_whitespace();
    let n = words.next().expect("n").parse::<i32>().expect("Entier");
    let mut x :i32;
    for _i in 0..n
    {
        x = words.next().expect("x").parse::<i32>().expect("Entier");
        if x.abs()%2==0
        {
            println!("{} is even",x);
        }
        else
        {
            println!("{} is odd",x);
        }
    }
    
   
}