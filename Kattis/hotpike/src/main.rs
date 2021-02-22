use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let words = input.split_whitespace();
    let mut day = 1;
    let mut max_temp = 10000;
    let mut best_day = 0;
    words.skip(1)//on élimine l'élément 1
        .map(|e| e.parse::<i32>().expect("Entier")) //on parse les entier
        .collect::<Vec<i32>>() //on transforme en vecteur d'entier
        .windows(3)//on groupe les éléments pas pack de 3 cf cours
        .for_each(|e| {
            let max = e[0].max(e[2]);
            if max < max_temp {
                max_temp = max;
                best_day = day;
            }
            day += 1;
        });
    println!("{} {}", best_day, max_temp);
}