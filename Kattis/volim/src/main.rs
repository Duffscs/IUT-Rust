use std::io;
use std::io::Read;

fn main() 
{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    
    let mut words = input.split_whitespace();
    let mut n =words.next().expect("n").parse::<i32>().expect("Entier");
    let q = words.next().expect("q").parse::<i32>().expect("Entier");

    let i : i32;
    i=0;
    let mut t : i32;
    t=0;
    while i<q && t < 210
    {
        t+= words.next().expect("t").parse::<i32>().expect("Entier");
        let txt = words.next().expect("txt");
        if txt == "T" && t<210
        {
            n=n+1;
            if n>8
            {
                n=1;
            }
        }
    }
    println!("{}",n);

}