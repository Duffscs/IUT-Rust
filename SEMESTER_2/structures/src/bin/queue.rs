use std::collections::VecDeque;

fn play_with_queue(){
    println!("File");
    let mut vec : VecDeque<u32> = VecDeque::new();
    vec.push_back(3);
    vec.push_back(1);
    vec.push_back(2);
    vec.push_back(4);
    for _ in 0..vec.len() {
        if let Some(element) = vec.pop_front() {
            println!("{}",element);
        }
    }
}

fn main() {
    play_with_queue();
}
