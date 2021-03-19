use std::collections::VecDeque;
use std::cmp::max;

fn main() {

}

fn plus_grande_somme_consecutive(vec: Vec<i32>) -> i32 {
    let mut temp = 0;
    return
        vec.iter().map(|e| {
            temp = max(e + temp, 0);
            return temp;
        }).fold(0, max);
}