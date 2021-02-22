use std::io;
use std::io::Read;

fn prime(nb : i32) -> bool
{
    let nb1= nb/2;
    let mut i=3;
    let mut val=true;
    if nb%2==0 && nb1*nb1 != nb
    {
        val=false;
    }
    while i<nb1+1 && val 
    {
        if nb%i==0
        {
            val=false;
        }
        i+=2;
    }
    return val;
}

fn primes(max : i32) -> Vec<i32>
{
    let mut vector = Vec::new();
    vector.push(2);
    let mut i=3;
    while i<=max
    {
        if prime(i)
        {
            vector.push(i);
        }
        i+=2;
    }
    return vector;
}

fn main() 
{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    
    let mut words = input.split_whitespace();
    let n =words.next().expect("n").parse::<i32>().expect("Entier");
    let mut vector = Vec::<i32>::new();
    let mut max:i32;
    max=0;
    for _ in 0..n
    {
        let j;
        j=words.next().expect("n").parse::<i32>().expect("Entier");
        vector.push(j);
        if j>max
        {
            max=j;
        }
    }
    
    let prim = primes(max);
    
    for i in 0..vector.len()
    {
        let mut chaine=String::new();
        let mut compte=0;
        for o in 0..prim.len()
        {
            if prim[o]<=vector[i]/2
            {
                for j in 0..prim.len()
                {
                    if prim[o]+prim[j]==vector[i]
                    {
                        let s= format!("{}+{}\n",prim[o],prim[j]);
                        chaine.push_str(&s);
                        compte+=1;
                    }
                    
                }
            }
        }
        println!("{} has {} representation(s)",vector[i],compte);
        println!("{}",chaine);
    }

}

