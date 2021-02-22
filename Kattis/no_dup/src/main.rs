use std::io;
use std::io::Read;
use std::collections::HashSet;
use std::process::exit;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut hash: HashSet<&str> = HashSet::new();
    input.split_whitespace().for_each(|e| {
        if !hash.contains(e){
            hash.insert(e);
        }else {
            println!("no");
            exit(0);
        }
    });
    println!("yes");
}