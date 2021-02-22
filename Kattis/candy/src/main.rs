use std::io;
use std::io::Read;

fn main()
{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut words = input.split_whitespace();
    let n = words.next().expect("n").parse::<u64>().expect("Entier");
    for _ in 0..n {
        let children = words.next().expect("n").parse::<u64>().expect("Entier");
        let mut sum: u64 = 0;
        for _ in 0..children {
            sum += words.next().expect("str").parse::<u64>().expect("u64") % children;
        }
        if sum % children as u64 == 0 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}