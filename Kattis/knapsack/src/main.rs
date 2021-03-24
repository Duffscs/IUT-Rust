use std::io::{stdin, Read};


fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).expect("Lecture de Stdin");
    let mut bool = true;
    let mut word = input.split_whitespace();
    while bool {
        let c: usize = word.next().expect("str").parse().expect("i32");
        let n: usize = word.next().expect("str").parse().expect("i32");
        let mut vec: Vec<(usize, usize)> = Vec::new();

        for _ in 0..n {
            let a: (usize, usize) = (word.next().unwrap().parse().unwrap(), word.next().unwrap().parse().unwrap());
            vec.push(a);
        }

        let vec = sac_a_dos(c, n, &mut vec);

        if vec.len() == 0 {
            println!("0");
        }else {
            println!("{}", vec.len());
            vec.iter().for_each(|e| print!("{} ", e));
            println!();
        }

        bool = word.clone().count() != 0;
    }
}

fn sac_a_dos(c: usize, n: usize, vec: &mut Vec<(usize, usize)>) -> Vec<usize> {
    let mut vm: Vec<Vec<usize>> = vec![vec![0; c + 1]; n + 1];
    let mut vm_pos: Vec<Vec<(Option<usize>, Option<usize>)>> = vec![vec![(None, None); c + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=c {
            let (vi, mi) = vec[i - 1];
            if mi > j {
                vm[i][j] = vm[i - 1][j];
                vm_pos[i][j] = vm_pos[i - 1][j]
            } else {
                let prev_vm = vm[i - 1][j];
                let next_vm = vm[i - 1][j - mi] + vi;
                if prev_vm >= next_vm {
                    vm[i][j] = prev_vm;
                    vm_pos[i][j] = vm_pos[i - 1][j]
                } else {
                    vm[i][j] = next_vm;
                    vm_pos[i][j] = (Some(i - 1), Some(j - mi));
                }
            }
        }
    }

    let mut vec: Vec<usize> = Vec::new();

    let mut i = vm_pos[n][c];
    while i.0 != None && i.1 != None {
        vec.push(i.0.unwrap());
        i = vm_pos[i.0.unwrap()][i.1.unwrap()];
    }
    vec
}
