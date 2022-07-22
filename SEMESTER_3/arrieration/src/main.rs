use std::io::{stdin, Read};
use std::collections::HashMap;

// fn main() {
//     let mut input = String::new();
//     stdin().read_to_string(&mut input).expect("lecture de stdin");
//     input.lines().for_each(|e| {
//         let mut vec: Vec<char> = Vec::new();
//         e.chars().for_each(|car| {
//             if car == '<' {
//                 vec.pop();
//             } else {
//                 vec.push(car);
//             }
//         });
//         vec.iter().for_each(|e| print!("{}", e)); //Pas de join pour le vecteur de caractère
//         println!();
//     });
// }


fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).expect("lecture de stdin");

    let mut map: HashMap<&str, i32> = HashMap::new();
    let mut vec: Vec<(&str, i32)> = Vec::new();
    let mut lines = input.lines();

    while let Some(e) = lines.next() {
        if e == "" {break;}
        let mut split = e.split_whitespace();
        let key = split.next().expect("read");
        let value: i32 = split.next().expect("read").parse().expect("i32");
        map.insert(key, value);
    }

    lines.for_each(|e| {
        let mut split = e.split_whitespace();
        let nom = split.next().expect("read");
        let mut score = 0;
        split.collect::<Vec<&str>>().chunks(2).for_each(|e| {
            let val = map.get(e[0]).unwrap();
            score += val * e[1].parse::<i32>().unwrap();
        });
        vec.push((nom, score));
    });

    vec.sort();
    for i in vec {
        if i.1 > 0 {
            println!("{} : personne saine", i.0);
        } else if i.1 < 0 {
            println!("{} : personne malade", i.0);
        } else {
            println!("{} : personne en danger", i.0);
        }
    }
}