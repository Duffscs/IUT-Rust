use std::io::{stdin, Read};
use std::collections::VecDeque;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).expect("read stdin");
    let longeur = input.len();
    let res = pls(input, longeur);
    println!("{}", 26 - res.len());
}

fn pls(string: String, indice: usize) -> String {
    let mut pls: Vec<(i32, usize)> = vec![(0, 0); indice];
    let vec_char: Vec<char> = string.chars().collect();
    if indice == 0 {
        return String::new();
    }
    pls[0] = (1, 0);

    for i in 1..indice {
        let mut max = (0, (1, 0));
        let mut j = 0;
        while j < i {
            if vec_char[j] < vec_char[i] && max.1 <= pls[j] {
                max = (j, pls[j]);
            }
            j += 1;
        }

        if vec_char[max.0] < vec_char[i] {
            pls[i] = (1 + (max.1).0, max.0);
        } else {
            pls[i] = (1, 0);
        }
    }

    let mut max = pls.iter().enumerate().max_by_key(|e| e.1).unwrap();

    let mut car: VecDeque<char> = VecDeque::new();
    while (max.1).0 != 1 {
        car.push_front(vec_char[max.0]);
        max = ((max.1).1, &pls[(max.1).1 as usize]);
    }
    car.push_front(vec_char[max.0]);
    return car.iter().collect::<String>();
}