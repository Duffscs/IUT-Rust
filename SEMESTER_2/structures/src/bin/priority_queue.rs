use priority_queue::PriorityQueue;

fn play_with_prioty_queue(){
    println!("PriorityQueue");
    let mut vec : PriorityQueue<u32,u32> = PriorityQueue::new();
    vec.push(3,2);
    vec.push(1,4);
    vec.push(2,3);
    vec.push(4,1);
    while !vec.is_empty(){
        if let Some((element,_)) = vec.pop(){
            println!("{}",element);
        }
    }
}

fn play_with_min_prioty_queue(){
    println!("PriorityQueue");
    let mut vec : PriorityQueue<u32,u32> = PriorityQueue::new();
    vec.push(3,3);
    vec.push(1,1);
    vec.push(2,2);
    vec.push(4,4);
    while !vec.is_empty(){
        if let Some((element,_)) = vec.pop(){
            println!("{}",element);
        }
    }
}

fn main() {
    play_with_prioty_queue();
    play_with_min_prioty_queue();
}
