use std::io;
use std::io::Read;


fn opperate(op1 : char,op2 : char,op3 : char) -> i32
{
    let mut nb1 = 4;
    let mut nb2 = 4;
    let mut nb3 = 4;
    let mut nb4 = 4;

    if op1=='-'
    {
        nb2=-1*nb2;
    }
    if op2=='-'
    {
        nb3=-1*nb3;
    }
    if op3=='-'
    {
        nb4=-1*nb4;
    }
    if op1=='*'
    {
        nb2=nb1*nb2;
        nb1=0;
    }
    if op1=='/'
    {
        nb2=nb1/nb2;
        nb1=0;
    }
    
    if op2=='*'
    {
        nb3=nb2*nb3;
        nb2=0;
    }
    if op2=='/'
    {
        nb3=nb2/nb3;
        nb2=0;
    }

    
    if op3=='*'
    {
        nb3=nb3*nb4;
        nb4=0;
    }

    if op3=='/'
    {
        nb3=nb3/nb4;
        nb4=0;
    }

    let nb=nb1+nb2+nb3+nb4;
    return nb;
}

fn main() 
{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut words = input.split_whitespace();
    let n =words.next().expect("n").parse::<i32>().expect("Entier");
    let mut nb : i32;
    let mut possibilites : Vec<String> = Vec::new();
    let mut num : Vec<i32> = Vec::new();
   
    
    possibilite(&mut possibilites,&mut num);
    for _ in 0..n
    {
        let mut boolean=false;
        nb =words.next().expect("n").parse::<i32>().expect("Entier");
        for i in 0..possibilites.len()
        {
            if nb==num[i]
            {
                println!("{}",possibilites[i]);
                boolean=true;
                break;
            }
        }
        if !boolean
        {
            println!("no solution");
        }
    }
    
}

fn possibilite(possibilites : &mut Vec<String>, possibilites_num : &mut Vec<i32>)
{
    let mut possibil : Vec<String> = Vec::new();
    let mut num : Vec<i32> = Vec::new();
    let op= vec!['-','+','*','/'];
    for o in 0..op.len()
    {
        for i in 0..op.len()
        {
            for j in 0..op.len()
            {
                let nb = opperate(op[o],op[i],op[j]);
                let s=format!("4 {} 4 {} 4 {} 4 = {}",op[o],op[i],op[j],nb);
                //println!("{}",s);
                possibil.push(s); 
                
                num.push(nb);
            }
        }
    }
    *possibilites=possibil;
    *possibilites_num=num;
}