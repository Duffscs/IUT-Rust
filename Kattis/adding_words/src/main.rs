use std::io;
use std::io::Read;
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut hash_map: HashMap<String, i64> = HashMap::new();
    input.lines().for_each(|line| {
        let mut string = line.to_string();
        if string.starts_with("def") {
            define(&mut hash_map, &mut string);
        } else if string.starts_with("calc") {
            calc(&mut hash_map, &mut string)
        } else if string.starts_with("clear") {
            hash_map = HashMap::new();
        }
    });
}

fn define(hash_map: &mut HashMap<String, i64>, line: &mut String) {
    let mut words = line.split_whitespace().skip(1);
    hash_map.insert(words.next().expect("name").to_string(), words.next().expect("valeur").parse().expect("i64"));
}

fn calc(mut hash_map: &mut HashMap<String, i64>, line: &mut String) {
    let sentence = line.replace("calc ", "");
    if let Some(resultat) = calculate(&mut hash_map, &mut sentence.clone()) {
        if let Some(variable) = find_key_for_value(&hash_map, resultat) {
            reponse(&sentence, variable);
        } else {
            reponse(&sentence, "unknown");
        }
    } else {
        reponse(&sentence, "unknown");
    }
}

fn reponse(sentence: &str, result: &str) {
    println!("{} {}", sentence, result);
}

fn calculate(hash_map: &mut HashMap<String, i64>, line: &mut String) -> Option<i64> {
    let mut words = line.split_whitespace();
    let mut calc = 0;
    let mut operator = "";
    while operator != "=" {
        if let Some(value) = is_value(hash_map, words.next().expect("str")) {
            if operator == "+" {
                calc += hash_map.get(value).unwrap() + 0;
            } else if operator == "-" {
                calc -= hash_map.get(value).unwrap() + 0;
            } else {
                calc += hash_map.get(value).unwrap() + 0;
            }
            operator = words.next().expect("str");
        } else {
            return None;
        }
    }
    Some(calc)
}

fn is_value<'a>(hash_map: &mut HashMap<String, i64>, value: &'a str) -> Option<&'a str> {
    if hash_map.contains_key(value) {
        return Some(value);
    } else {
        return None;
    }
}

fn find_key_for_value(map: &HashMap<String, i64>, value: i64) -> Option<&String> {
    map.iter().find_map(|(key, &val)| if val == value { Some(key) } else { None })
}