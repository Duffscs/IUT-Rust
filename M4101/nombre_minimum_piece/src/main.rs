use std::cmp::min;
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).expect("Lecture stdin");
    input.lines().skip(1)
        .collect::<Vec<&str>>()
        .chunks(2).for_each(|e| {
        let max_s: usize = e[1].parse().expect("usize");
        let vec: Vec<usize> = e[0].split_whitespace().map(|e| e.parse::<usize>().expect("usize")).collect();
        let mut nmp_ = nmp(vec, max_s);
        if nmp_ == Some(i32::max_value()) {
            println!("Impossible")
        }else if nmp_ == Some(0) {
            println!("0")
        }else {
            println!("{}",nmp_.unwrap());
        }
    });

}

fn nmp(vec: Vec<usize>, max_s: usize) -> Option<i32> {
    let mut nmp = vec![0; max_s + 1];
    for s in 1..=max_s {
        let mut val = i32::max_value() - 1;
        for i in 0..vec.len() {
            if vec[i] <= s {
                val = min(val, nmp[s - vec[i]]);
            }
        }
        val = val + 1;
        nmp[s] = val;
    }
    return nmp.pop();
}
