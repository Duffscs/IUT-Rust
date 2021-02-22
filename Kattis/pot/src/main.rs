use std::io;
use std::io::Read;

fn main()
{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let somme : u64 = input.split_whitespace().skip(1).map(|e| {
        let mut string = e.to_string();
        let pow = string.pop().unwrap().to_string().parse::<u32>().expect("u64");
        string.parse::<u64>().expect("i64").pow(pow)
    }).sum();
    println!("{}", somme);
}