use std::io;
use std::io::Read;
use std::collections::{VecDeque, HashMap, HashSet};
use std::str::Chars;


fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    input.lines()
        .skip(1)
        .collect::<Vec<&str>>()
        .chunks(3)
        .for_each(|element| {
            let instruction = element[0].chars();
            let nb_elem = element[1].parse::<usize>().expect("i32");
            let mut nb_pop_front = 0;
            let mut nb_pop_back = 0;
            let mut rev = true;
            count_pop(instruction, &mut nb_pop_front, &mut nb_pop_back, &mut rev);
            if nb_elem > (nb_pop_front + nb_pop_back) {
                let mut array_string = element[2].to_string();
                let mut vec = string_to_vec(&mut array_string);
                vec = remove_element_front(vec, nb_pop_front);
                vec = remove_element_back(vec, nb_pop_back);
                afficher_vector(& mut vec, rev);
            } else if nb_elem == (nb_pop_front + nb_pop_back) {
                println!("[]");
            } else {
                println!("error");
            }
        });
}

fn count_pop(instruction: Chars, nb_front: &mut usize, nb_back: &mut usize, rev: &mut bool) {
    instruction.for_each(|e| {
        if e == 'D' {
            if *rev {
                *nb_front += 1;
            } else {
                *nb_back += 1;
            }
        } else {
            *rev = !*rev;
        }
    });
}

fn remove_element_front(mut vec: VecDeque<&str>, nb_pop : usize) -> VecDeque<&str>{
    for _ in 0..nb_pop {
        vec.pop_front();
    }
    return vec;
}

fn remove_element_back(mut vec: VecDeque<&str>, nb_pop : usize) -> VecDeque<&str> {
    for _ in 0..nb_pop {
        vec.pop_back();
    }
    return vec;
}

fn string_to_vec(array_string: &mut String) -> VecDeque<&str> {
    array_string.trim();
    array_string.remove(0);
    array_string.pop();
    array_string.split(',').collect::<VecDeque<&str>>()
}

fn afficher_vector(mut vec: &mut VecDeque<&str>, rev : bool) {
    if rev {
        afficher_vector_alendroit(&mut vec);
    } else {
        afficher_vector_alenver(&mut vec);
    }
}

fn afficher_vector_alenver(vec: &mut VecDeque<&str>) {
    let mut string = String::new();
    string.push('[');
    let mut o = (vec.len() - 1) as i32;
    for _ in 0..vec.len() {
        string += vec[o as usize];
        string.push(',');
        o -= 1;
    }
    if string.len() != 1 {
        string.pop();
    }
    string.push(']');
    println!("{}", string);
}

fn afficher_vector_alendroit(vec: &mut VecDeque<&str>) {
    let mut string = String::new();
    string.push('[');
    for i in vec {
        string += i;
        string.push(',');
    }
    if string.len() != 1 {
        string.pop();
    }
    string.push(']');
    println!("{}", string);
}
