use std::io;
use std::io::Read;
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut hash_map: HashMap<&str, i32> = HashMap::new();
    let mut max = 0;
    let mut max_name = "";
    let mut nb_max = 0;
    input.lines()
        .for_each(|e| {
            if e == "***" { return; }
            let mut score = 1;
            if hash_map.contains_key(e.clone()) {
                score = hash_map.get(e.clone()).unwrap() + 1;
            }
            hash_map.insert(e.clone(), score);
            if score > max {
                max = score;
                max_name = e.clone();
                nb_max = 1;
            } else if score == max {
                nb_max += 1;
            }
        });
    if nb_max > 1 {
        println!("Runoff!");
    } else {
        println!("{}", max_name);
    }
}