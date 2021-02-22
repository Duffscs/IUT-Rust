use std::io;
use std::io::Read;
use std::collections::HashMap;
use std::collections::hash_map::RandomState;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let hash_map: HashMap<&str, &str> = get_alphabet();
    input.lines().next().expect("str").chars().for_each(|char| {
        if let Some(element) = hash_map.get(char.to_string().to_lowercase().as_str()) {
            print!("{}", element);
        } else {
            print!("{}", char);
        }
    });
}

fn get_alphabet () -> HashMap<&'static str, &'static str> {
    let mut hash_map: HashMap<&str, &str> = HashMap::new();
    hash_map.insert("a", "@");
    hash_map.insert("n", "[]\\[]");
    hash_map.insert("b", "8");
    hash_map.insert("o", "0");
    hash_map.insert("c", "(");
    hash_map.insert("p", "|D");
    hash_map.insert("d", "|)");
    hash_map.insert("q", "(,)");
    hash_map.insert("e", "3");
    hash_map.insert("r", "|Z");
    hash_map.insert("f", "#");
    hash_map.insert("s", "$");
    hash_map.insert("g", "6");
    hash_map.insert("t", "']['");
    hash_map.insert("h", "[-]");
    hash_map.insert("u", "|_|");
    hash_map.insert("i", "|");
    hash_map.insert("v", "\\/");
    hash_map.insert("j", "_|");
    hash_map.insert("w", "\\/\\/");
    hash_map.insert("k", "|<");
    hash_map.insert("x", "}{");
    hash_map.insert("l", "1");
    hash_map.insert("y", "`/");
    hash_map.insert("m", "[]\\/[]");
    hash_map.insert("z", "2");
    return hash_map;
}