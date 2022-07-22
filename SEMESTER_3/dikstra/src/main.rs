use std::io;
use std::io::Read;

fn minimum(sommet : &Vec<i32>, tableau : &[i32]) -> i32 { 
    let mut min :i32 = 200000;
    let mut indice : i32 = 0;
    for i in 0..tableau.len() as i32{
        if min > tableau[i as usize] && !sommet.contains(&i) && tableau[i as usize] != 0{
            min= tableau[i as usize];
            indice = i as i32;
        }
    }
    return indice;
}


fn adja(liste1 : &mut Vec<i32>, liste2 : &mut Vec<i32>, liste3 : &mut Vec<i32>, nbsommet : i32, words : &mut std::str::SplitWhitespace<'_> ){

    let mut somme = 0;
    for _ in 0..nbsommet{
        let nb = words.next().expect("nb").parse::<i32>().expect("entier");
        liste1.push(nb);
        somme = nb-1;
    }
    initl2l3(liste2, liste3, somme, words);
}

fn insid(liste1 : &mut Vec<i32>, liste2 : &mut Vec<i32>, liste3 : &mut Vec<i32>, nbsommet : i32, words : &mut std::str::SplitWhitespace<'_>){
    let mut somme = 0;
    liste1.push(1);    
    for _ in 0..nbsommet{
        let nb =words.next().expect("nb").parse::<i32>().expect("entier");
        somme += nb;
        liste1.push(somme+1);
    }
    initl2l3(liste2, liste3, somme, words);
}

fn matrix (liste1 : &mut Vec<i32>, liste2 : &mut Vec<i32>, liste3 : &mut Vec<i32>, nbsommet : i32, words : &mut std::str::SplitWhitespace<'_>){
    liste1.push(1);
    
    println!();
    let mut nbelem = 1;
    for _ in 1..nbsommet+1{
        for o in 1..nbsommet+1{
            let nb = words.next().expect("nb").parse::<i32>().expect("entier");
            if nb!= 0 {
                    liste2.push(o);
                    liste3.push(nb);
                    nbelem+=1;
                }
        }
        let g : usize = (liste1.len()-1) as usize;
        if nbelem as usize == liste1[g] as usize{
            liste1.push(nbelem+1);
            liste2.push(-1);
            liste3.push(-1);
        }else {liste1.push(nbelem);}         
    }
}

fn initl2l3(liste2 : &mut Vec<i32>, liste3 : &mut Vec<i32>, somme : i32, words : &mut std::str::SplitWhitespace<'_> ){
    for _ in 0..somme+1{
        liste2.push(words.next().expect("nb").parse::<i32>().expect("entier"));
    }
    for _ in 0..somme+1{
        liste3.push(words.next().expect("nb").parse::<i32>().expect("entier"));
    }
}



fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut words = input.split_whitespace();
    let tipe = words.next().expect("string");
    let nbsommet = words.next().expect("nb").parse::<i32>().expect("entier");

    let mut liste1 : Vec<i32> = Vec::new();
    let mut liste2 : Vec<i32> = Vec::new();
    let mut liste3 : Vec<i32> = Vec::new();
    liste1.push(0);
    liste2.push(0);
    liste3.push(0);

    if tipe == "insid" {
        insid( &mut liste1, &mut liste2, &mut liste3, nbsommet, &mut words);
    }else if tipe == "adja" {
            adja( &mut liste1, &mut liste2, &mut liste3, nbsommet, &mut words);
    }else if tipe == "matrix"{
         matrix( &mut liste1, &mut liste2, &mut liste3, nbsommet, &mut words);
    }else{
        println!();
        eprintln!("préciser insid ou adjacence");
        println!();
        std::process::exit(1);
    }

    println!("{:?}",liste1);
    println!("{:?}",liste2);
    println!("{:?}",liste3);
    
    let mut sommet : Vec<i32> = Vec::new();
    let mut c = vec![0;(nbsommet+1) as usize];
    let mut d = vec![0;(nbsommet+1) as usize];
    
    let mut i =1;
    let mut stringC = String::new();
    let mut stringD = String::new();
   
    sommet.push(0);
    println!();
    for _ in 0..(nbsommet-1) as usize{
        for o in liste1[i] as usize ..liste1[i+1] as usize{
            let id = liste2[o as usize] as usize;
            let e = d[i]+liste3[o];
            
            if e < d[id] || d[id]==0{
                d[id] = e;
                c[id] = i;
            }
        }
        sommet.push(i as i32);
        stringC.push_str(format!("{}  {:?} \n",i,c).as_str());
        stringD.push_str(format!("{}  {:?} \n",i,d).as_str());
        i = minimum(& sommet, & d) as usize;
        if i==nbsommet as usize{
            sommet.push(i as i32);
            stringC.push_str(format!("{}  {:?} \n",i,c).as_str());
            stringD.push_str(format!("{}  {:?} \n",i,d).as_str());
            i = minimum(& sommet, & d) as usize;
        }
    }
    println!();
    println!("{}",stringD);

    println!();
    println!("{}",stringC);

    print!("  [{},",0);
    for i in 1..nbsommet{
        print!(" {},",i);
    }
    println!("{}]",nbsommet);
    println!("  {:?}",c);
}
