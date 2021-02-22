use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let nb : i32 = input.trim().parse().expect("Entier");
    for i in 1..nb+1
    {
        println!("{} Abracadabra",i);
    }
   
}
