use std::io;
use std::io::Read;

fn main() 
{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut words = input.split_whitespace();
    let mut h=words.next().expect("h").parse::<i32>().expect("Entier");
    let mut m=words.next().expect("m").parse::<i32>().expect("Entier");
    if m<45
    {
        h=h-1;
        if h==-1
        {
            h=23;
        }
        m=m+15;
    }
    else
    {
        m=m-45;
    }
    println!("{} {}",h,m);
}