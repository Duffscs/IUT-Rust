use std::io;
use std::io::Read;
use std::time::Instant;

fn is_prime(prime: &Vec<u64>, index: u64) -> bool {
    let vec_index = (index % 64) as u32;
    let vec_number = (index / 64) as usize;
    return prime[vec_number] & deux(vec_index) == 0;
}

fn not_prime(prime: &mut Vec<u64>, index: u64) {
    let vec_index = (index % 64) as u32;
    let vec_number = (index / 64) as usize;
    prime[vec_number] += deux(vec_index);
}

fn deux(pow: u32) -> u64 {
    let deux = 1;
    return deux << pow;
}

fn cocher_multiple(nombre: u64, borne: u64, prime: &mut Vec<u64>) {
    let mut multiple = nombre * nombre;
    while multiple <= borne {
        if is_prime(prime, multiple) {
            not_prime(prime, multiple)
        }
        multiple += nombre;
    }
}

fn compter_premier(borne1: u64, borne2: u64, prime: &Vec<u64>) -> u64 {
    let mut compte = 0;
    let mut i = borne1;
    if i & 1 == 0 { i += 1; } //Si i est paire on le rend impaire
    while i <= borne2 {
        if is_prime(prime, i) {
            compte += 1;
        }
        i += 2;
    }
    return compte;
}

fn primes(max: u64) -> Vec<u64> {
    let mut prime: Vec<u64> = vec![0; ((max / 64) + 1) as usize];
    prime[0] += 3;
    let racine = ((max as f64).sqrt() + 1.0) as u64;
    cocher_multiple(2, max, &mut prime);
    let mut compte = 1;
    let mut i: u64 = 3;
    while i <= racine {
        if is_prime(&mut prime, i) {
            cocher_multiple(i, max, &mut prime);
            compte += 1;
        }
        i += 2;
    }
    compte += compter_premier(racine + 1, max, &prime);
    println!("{}", compte);
    return prime;
}

fn main() {
    let now = Instant::now();
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut words = input.split_whitespace();
    let max = words.next().expect("max").parse::<u64>().expect("Entier");
    let prime = primes(max);
    words.skip(1).for_each(|e| {
        if is_prime(&prime, e.parse::<u64>().expect("u64")) {
            println!("1");
        } else {
            println!("0");
        }
    });
    // println!("{:?}", prime);
    println!("{}", now.elapsed().as_millis());
}