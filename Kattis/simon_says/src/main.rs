use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    input.lines().skip(1).for_each(|e| {
        if !e.starts_with("Simon says "){ return; }
        println!("{}", e.replace("Simon says ",""));
    })
}