use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut words = input.split_whitespace();
    let x = words.next().expect("x").parse::<i32>().expect("Entier");
    let y = words.next().expect("y").parse::<i32>().expect("Entier");
    let z = words.next().expect("z").parse::<i32>().expect("Entier");
    for i in 1..z+1
    {
        if i%x==0 && i%y==0
        {
            println!("FizzBuzz");
        }
        else
        {
            if i%x==0
            {
                println!("Fizz");
            } 
            else
            {
                if i%y==0
                {
                    println!("Buzz");
                }
                else
                {
                    println!("{}",i);
                }
            }
        }
    }
}