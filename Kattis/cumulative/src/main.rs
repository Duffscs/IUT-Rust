use std::io::{stdin, Read};
use std::cmp::max;
use std::collections::{VecDeque, HashSet, HashMap};
use std::time::Instant;

fn main() {
    // let mut input = String::new();
    // stdin().read_to_string(&mut input).expect("Lecture de stdin");
    // let mut entrer = input.lines().skip(1).map(|line| line.parse::<u64>().expect("i32")).collect::<Vec<u64>>();
    // let mut max = *entrer.iter().max().unwrap();
    // let mut hashset : HashSet<u64> = entrer.clone().into_iter().collect();
    // let mut hashMap : HashMap<u64,u64> = HashMap::new();
    // let mut a = 1;
    // let mut sum = 1;
    // hashMap.insert(0,0);
    // hashMap.insert(1,1);
    let mut a = Instant::now();
    for i in 0..100000000 {
        (i, sum_of_digits(i as u64));
    }
    println!("{:?}", a.elapsed().as_millis());

    a = Instant::now();
    let mut hashMap : HashMap<u64,u64> = HashMap::new();
    for i in 0..100000000 {
        (i, sod(i as u64, & mut hashMap));
    }
    println!("{:?}", a.elapsed().as_millis());
    // for i in 2..=max {
    //     a = a + sum_of_digits(a);
    //     sum = (a + sum) % 1000000007;
    //     if hashset.contains(&i) {
    //         hashMap.insert(i,sum);
    //     }
    // }
    // entrer.iter().for_each(|e| {
    //     println!("{}",hashMap.get(e).unwrap());
    // })
}

struct DigitIter(u64);

impl Iterator for DigitIter {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 { return None; }
        let ret = self.0 % 10;
        self.0 = self.0 / 10;
        Some(ret)
    }
}

fn sum_of_digits(entier: u64) -> u64 {
    return DigitIter(entier).sum::<u64>();
}

fn sod(number: u64, map: & mut HashMap<u64, u64>) -> u64 {
    if number == 0 {
        return number;
    }
    if map.contains_key(&number) {
        return *map.get(&number).unwrap();
    }
    let tmp = sod(number / 10, map);
    map.insert(number, number % 10 + tmp);
    return *map.get(&number).unwrap();
}
