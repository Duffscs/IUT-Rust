use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut words = input.split_whitespace();
    let mut somme = 0;
    words.skip(1)
        .map(|e| e.parse::<i32>().expect("Entier"))
        .filter(|e| {
            return *e < 0;
        })
        .for_each(|e| {
            somme += -e;
        });
    println!("{}",somme);
}
