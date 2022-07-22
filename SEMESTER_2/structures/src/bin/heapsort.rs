use priority_queue::PriorityQueue;

fn heapsort(tableau : &mut Vec<i32>) {
    let mut pq : PriorityQueue<i32,i32> = PriorityQueue::new();
    while !tableau.is_empty(){
        if let Some(element) = tableau.pop(){
            pq.push(element,-element);
        }
    }
    while !pq.is_empty(){
        if let Some((element,_)) = pq.pop(){
            tableau.push(element);
        }
    }
    return ;
}

fn main() {
    let mut vec = vec![3,1,5,8,2,4];
    println!("{:?}",vec);
    heapsort(&mut vec);
    println!("{:?}",vec);
}