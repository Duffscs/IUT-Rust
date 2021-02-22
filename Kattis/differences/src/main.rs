use std::io;
use std::io::Read;

fn main() 
{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut words = input.split_whitespace();
    let mut nb=words.next().expect("h").parse::<i32>().expect("Entier");
    let mut l : &str;
    let mut li : &str;
    for _ in 0..nb
    {
        l=words.next().expect("l1");
        li=words.next().expect("l2");
        println!("{}",l);
        println!("{}",li);
        let a : Vec<char> = l.chars().collect();
        let b : Vec<char> = li.chars().collect();
        for i in 0..a.len()
        {
            if a[i] == b[i]
            {
                print!(".");
            }
            else
            {
                print!("*");
            }
        }
        println!("");
        
    }
}