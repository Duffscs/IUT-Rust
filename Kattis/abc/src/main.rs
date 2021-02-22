use std::io;
use std::io::Read;
use std::collections::{VecDeque, HashMap};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut lines = input.lines();
    let mut line1 = lines.next().expect("&str").split_whitespace().map(|e| e.parse().expect("i32")).collect::<Vec<i32>>();
    let line2 = lines.next().expect("&str").chars().collect::<Vec<char>>();
    let entry = ['A', 'B', 'C'];
    let mut hash_map: HashMap<char, i32> = HashMap::new();
    for i in 0..3 {
        let min = get_min_vec(&line1);
        hash_map.insert(entry[i], min.1 + 0);
        line1.remove(min.0);
    }
    line2.iter().for_each(|e| {
        print!("{} ", hash_map.get(e).unwrap());
    });


}

fn get_min_vec(vec :& Vec<i32>) -> (usize, i32){
    let mut min = 101;
    let mut index = 0;
    for i in 0..vec.len() {
        if vec[i] < min {
            min = vec[i];
            index = i;
        }
    }
    return (index, min);
}