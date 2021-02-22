use std::io;
use std::io::Read;

fn div(totalleg : i32, leg : i32 ) -> i32
{
    let mut res : i32;
    res = totalleg/leg;
    let h = totalleg as f64 /leg as f64;
    if h>res as f64
    {
        res+=1;
    }
    return res;
}

fn main() 
{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    
    let mut words = input.split_whitespace();
    let b =words.next().expect("b").parse::<i32>().expect("Entier");
    let d =words.next().expect("d").parse::<i32>().expect("Entier");
    let c =words.next().expect("c").parse::<i32>().expect("Entier");
    let l =words.next().expect("l").parse::<i32>().expect("Entier");
    let mut comp =false;
    for o in 0..div(l,b)+1
    {
        for i in 0..div(l,d)+1
        {
            
            for j in 0..div(l,c)+1
            {
                let a = o*b + i*d + j*c;
                if a==l
                {
                    println!("{} {} {}",o,i,j);
                    comp=true;
                }

            }
        }
    }
    if !comp
    {
        print!("impossible");
    }

}
