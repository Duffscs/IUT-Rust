use std::io;
use std::io::Read;
use std::collections::{HashSet};

fn main() {
    let mut str = String::new();
    io::stdin().read_to_string(&mut str).expect("Lecture de stdin");
    let alphabet = "abcdefghijklmnopqrstuvwxyz".chars().collect::<HashSet<char>>();
    str.lines().into_iter().for_each(|x| {
        let mut liste_lettes = alphabet.clone();
        x.replace(" ","").to_lowercase().chars().for_each(|car| {
            liste_lettes.remove(&car);
        });
        if liste_lettes.len() == 0 {
            println!("PANGRAMME");
        } else {
            let mut vec = liste_lettes.into_iter().collect::<Vec<char>>();
            vec.sort();
            vec.iter().for_each(|car| print!("{}",car));
            println!();
        }

    });
}

