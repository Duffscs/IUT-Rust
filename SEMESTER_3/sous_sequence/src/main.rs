fn main() {
    let res = pls("bonjour".to_string(),7);
    println!("{}", res);
}

fn pls(string: String, indice: i32) -> String {
    let mut vec: Vec<(i32, i32)> = Vec::new();
    let vec_char: Vec<char> = string.chars().collect();
    if indice < 0 {
        return String::new();
    }
    if indice == 0 {
        return string.chars().take(0).collect();
    }
    let longueur = 1;
    vec.push((longueur, 0));

    for i in 1..indice {
        let car = vec_char[i as usize];
        let a = vec.pop().unwrap();
        let last_char = vec_char[a.1 as usize];
        if car > last_char {
            vec.push(a);
            vec.push((a.0 + 1, i));
        } else {
            vec.push((a.0, i))
        }
    }

    return vec.iter().map(|e| {
        return vec_char[e.1 as usize];
    }).collect();
}