use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    input.lines().skip(1).for_each(|e| {
        let mut words = e.split_whitespace();
        let beats: f64 = words.next().expect("str").parse().expect("f64");
        let time: f64 = words.next().expect("str").parse().expect("f64");
        let freq = 60.0 / time;
        let beat_rate = beats * freq;
        println!("{} {} {}", beat_rate - freq, beat_rate, beat_rate + freq);
    });
}
