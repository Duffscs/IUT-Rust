use std::io;
use std::io::Read;
use std::collections::{HashMap, HashSet};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut user_list: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut dictionary: HashMap<&str, i64> = HashMap::new();
    input.lines()
        .skip(1) //on ignore la première ligne
        .for_each(|e| { // Pour chaque ligne
            let mut words = e.split_whitespace();
            let username = words.next().expect("str");
            if_not_in_add_user(&mut user_list, username);
            for mot in words {
                add_one_occurence(&mut dictionary, mot);
                user_list.get_mut(username).unwrap().insert(mot);
            }
        });
    let intersect = common_words(&mut user_list);
    if intersect.len() == 0 {
        println!("ALL CLEAR");
    }else {
        let mut res = intersect.into_iter()
            .map(|e| (-dictionary.get(e).unwrap(), e))
            .collect::<Vec<(i64, &str)>>();
        res.sort();
        res.iter().for_each(|e| println!("{}", e.1));
    }
}

fn common_words<'a>(hashmap: &mut HashMap<&'a str, HashSet<&'a str>>) -> HashSet<&'a str> {
    let mut intersect: HashSet<&'a str> = HashSet::new();
    let mut idx = 0;
    for (_, mots) in hashmap {
        if idx == 0 { intersect = mots.clone(); idx+=1; }
        intersect = intersect.intersection(&mots).into_iter().map(|e| *e).collect();
    }
    intersect
}

fn add_one_occurence<'a>(occurence: &mut HashMap<&'a str, i64>, mot: &'a str) {
    let mut nb_occurence: i64 = 1;
    if occurence.contains_key(mot) {
        nb_occurence = occurence.get(mot).unwrap() + 1;
    }
    occurence.insert(mot, nb_occurence);
}

fn if_not_in_add_user<'a>(hashmap: &mut HashMap<&'a str, HashSet<&str>>, username: &'a str) {
    if !hashmap.contains_key(username) {
        hashmap.insert(username, HashSet::new());
    }
}