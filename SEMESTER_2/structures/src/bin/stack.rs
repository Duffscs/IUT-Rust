fn play_with_stack(){
    println!("Stack");
    let mut vec = vec![3,1,2,4];
    for _ in 0..vec.len() {
        if let Some(element) = vec.pop() {
            println!("{}",element);
        }
    }
}

fn main() {
    play_with_stack();
}
