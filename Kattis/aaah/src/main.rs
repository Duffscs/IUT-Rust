use std::io;
use std::io::Read;

fn main() 
{
    //let op= vec!['*','/','+','-'];
    
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let words = input.lines();
    let vector: Vec<&str>= words.collect();
    //println!("{} {}",vector[0].len(),vector[1].len());
    if vector[1].len() <= vector[0].len()
    {
        println!("go");
    }
    else
    {
        println!("no");
    }
    

}
