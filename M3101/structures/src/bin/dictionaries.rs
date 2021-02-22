use std::collections::{hash_set, HashSet, HashMap};

fn play_with_set(){
    let mut hash: HashSet<i32> = HashSet::new();
    hash.insert(3);
    hash.insert(1);
    hash.insert(2);
    hash.insert(4);
    for elem in hash.iter(){
        println!("{}",elem);
    }
}

fn play_with_map(){
    let mut hash: HashMap<i32,&str> = HashMap::new();
    hash.insert(3,"trois");
    hash.insert(1,"un");
    hash.insert(2,"deux");
    hash.insert(4,"quatre");
    for elem in hash.iter(){
        println!("{}",elem.1);
    }
}

fn main(){
    play_with_set();
    play_with_map();
}