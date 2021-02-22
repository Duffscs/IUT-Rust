use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut words = input.split_whitespace();
    let n = words.next().expect("n").parse::<i32>().expect("Entier");
    let mut r : i32;
    let mut e : i32 ;
    let mut c : i32 ;
    
    for _i in 1..n+1
    {
        r=words.next().expect("r").parse::<i32>().expect("Entier");
        e=words.next().expect("e").parse::<i32>().expect("Entier");
        c=words.next().expect("c").parse::<i32>().expect("Entier");
        r=r+c;
        if r<e
        {
            println!("advertise");
        }
        else
        {
            if r>e
            {
                println!("do not advertise");
            }
            else
            {
                println!("does not matter");
            }
        }
    }
}