use std::io;
use std::io::{Read, BufRead};
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    // let mut hash_map : HashMap<u64,u64> = HashMap::new();
    io::stdin().lock().lines().for_each(|e| {
        let line: u64 = e.unwrap().parse().unwrap();
        let now = Instant::now();
        let a = fibonnacci3(line);
        println!("temps : {}", now.elapsed().as_micros());
        println!("{}", a);
    });
}

fn fibonacci1(n: u64) -> u64 {
    if n == 0 { return 0; }
    if n == 1 { return 1; }
    return fibonacci1(n - 1) + fibonacci1(n - 2);
}

fn fibonacci2(n: u64, mut hash: &mut HashMap<u64, u64>) -> u64 {
    if n == 0 { return 0; }
    if n == 1 { return 1; }
    if !hash.contains_key(&n) {
        let a = (fibonacci2(n - 1, &mut hash) + fibonacci2(n - 2, &mut hash)) % 123456789;
        hash.insert(n, a);
    }
    return *hash.get(&n).unwrap();
}

fn fibonnacci3(n: u64) -> u64 {
    let mut n1: u64 = 0;
    let mut n2: u64 = 1;
    let mut nb = n / 2;
    if n % 2 == 1 { nb += 2; }
    for _ in 0..nb {
        n1 = (n1 + n2) % 123456789;
        n2 = (n1 + n2) % 123456789;
    }
    return if n % 2 == 1 { n2 } else { n1 };
}