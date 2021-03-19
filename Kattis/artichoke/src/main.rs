use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).expect("str");
    let mut words = input.split_whitespace();
    let func = words.clone().map(|e| e.parse::<f64>().unwrap()).collect::<Vec<f64>>().chunks(5).map(|e| (e[0],e[1],e[2],e[3],e[4]) ).next().unwrap();
    let n: f64 = words.skip(5).next().expect("str").parse().expect("i32");
    let res = (1..=n as i32).map(|x| fx(x as f64, func)).collect::<Vec<f64>>();
    let somme = somme_element_de_proche_en_proche(res);
    println!("{}", plus_grande_somme_consecutive(somme));
}

fn fx(x: f64, (p, a, b, c, d): (f64, f64, f64, f64, f64)) -> f64 {
    return p * ((a * x + b).sin() + (c * x + d).cos() + 2.0);
}

fn somme_element_de_proche_en_proche(vec: Vec<f64>) -> Vec<f64> {
    return vec.windows(2).map(|i| i[0] - i[1]).collect();
}

fn plus_grande_somme_consecutive(vec: Vec<f64>) -> f64 {
    let mut temp = 0.00;
    return
        vec.iter().map(|e| {
            temp = f64::max(e + temp, 0.00);
            return temp;
        }).fold(0.00, f64::max);
}