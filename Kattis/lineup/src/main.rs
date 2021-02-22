use std::io;
use std::io::Read;

fn main() 
{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    
    let mut words = input.split_whitespace();
    let n =words.next().expect("n").parse::<i32>().expect("Entier");
    let mut q ;
    let mut vector = Vec::new();
    vector.push(1);
    for _ in 0..n-1
    {
       q = words.next().expect("q").parse::<i32>().expect("Entier");
       vector.push(q+2);
    }
    //println!("{:?}",vector);
    let mut min = 1;
    for _ in 0..vector.len()
    {
        let mut i=0;
        while vector[i]!=min
        {
            i+=1;
        }
        print!("{} ",i+1);
        min+=1;
    }

}