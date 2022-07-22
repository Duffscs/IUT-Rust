use std::io;
use std::io::Read;
use std::ops::Add;

fn main() {
    let mut str = String::new();
    io::stdin().read_to_string(&mut str).expect("Lecture de stdin");
    let mut words = str.split_whitespace();
    let dim1: usize = words.next().expect("str").parse().expect("i32");
    let dim2: usize = words.next().expect("str").parse().expect("i32");
    let mat1 = read_matrix(words.clone().collect::<Vec<&str>>().join(" "), dim1, dim2);
    for _ in 0..(dim1 * dim2) { words.next(); }
    let dim3: usize = words.next().expect("str").parse().expect("i32");
    let dim4: usize = words.next().expect("str").parse().expect("i32");
    let mat2 = read_matrix(words.clone().collect::<Vec<&str>>().join(" "), dim3, dim4);

    if dim2 != dim3 {
        println!("Produit impossible");
        return;
    }
    matrix_product(mat1, mat2);
}

fn matrix_product(matrix1: Vec<Vec<i32>>, mut matrix2: Vec<Vec<i32>>) {
    let mut res = vec![vec![0; matrix2[0].len()]; matrix1.len()];
    for ligne in 0..res.len() {
        for colonne in 0..res[0].len() {
            for i in 0..matrix2.len() { // ou matix1[0].len
                res[ligne][colonne] += matrix1[ligne][i] * matrix2[i][colonne];
            }
        }
    }
    print_matrix(res);
}

fn print_matrix(matrix1: Vec<Vec<i32>>) {
    println!("{} {}", matrix1.len(), matrix1[0].len());
    let mut result = String::new();
    for i in 0..matrix1.len() {
        for j in 0..matrix1[0].len() {
            result = result.add(format!("{} ", matrix1[i][j]).as_str());
        }
        result.pop();
        result = result.add("\n");
    }
    println!("{}", result);
}

fn read_matrix(string: String, dim1: usize, dim2: usize) -> Vec<Vec<i32>> {
    let mut words = string.split_whitespace().take(dim1 * dim2);
    let mut result = vec![vec![0; dim2]; dim1];
    for i in 0..dim1 {
        for j in 0..dim2 {
            result[i][j] = words.next().expect("str").parse().expect("i32");
        }
    }
    result
}