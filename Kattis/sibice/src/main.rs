use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut words = input.split_whitespace().map(|e| e.parse::<f64>().expect("f64"));
    let nb_matches = words.next().expect("f64");
    let width_square = words.next().expect("f64").powf(2.0);
    let height_square = words.next().expect("f64").powf(2.0);
    let hypothenuse = (width_square + height_square).sqrt();
    words.for_each(|matche| {
        if matche <= hypothenuse {
            println!("DA");
        }else {
            println!("NE");
        }
    })


}