use std::io;
use std::io::Read;


fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let words = input.split_whitespace();
    words.skip(1).for_each(|e| {
        let columns = (e.clone().chars().count() as f64).sqrt() as usize;
        let mut vec: Vec<Vec<char>> =vec![Vec::new(); columns];
        let mut idx = 0;
        e.chars().for_each(|car| {
            vec[idx % columns].push(car);
            idx += 1;
        });
        vec.reverse();
        for i in vec {
            for o in i {
                print!("{}", o);
            }
        }
        println!();
    });
}
