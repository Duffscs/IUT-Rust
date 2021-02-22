use std::io;
use std::io::Read;
extern crate multimap;
use multimap::MultiMap;

fn main() 
{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut words = input.split_whitespace();
    let nbpers = words.next().expect("nbpers").parse::<i32>();
    let time = words.next().expect("time").parse::<i32>().unwrap();
    let vec : Vec<&str>  = words.collect();
    let mut queu : MultiMap<i32,i32> = MultiMap::new();
    for i in 0..vec.len()/2
    {
        queu.insert(vec[i*2+1].parse::<i32>().unwrap(),vec[i*2].parse::<i32>().unwrap());
    }

    for i in 0..time
    {
        for (key,value) in &queu
        {
            println!("{}: {:?}", key, value);
        }
        println!();
        ecoulertemps(&mut queu,i);
    }

    //removek(2,0 , &mut queu);

    
}

fn removek(key : i32, index : usize, map : &mut MultiMap<i32,i32> )
{
    let tab = map.get_vec_mut(&key).unwrap();
    tab.remove(index);
}

fn ecoulertemps(map : &mut MultiMap<i32,i32>, index : i32 )
{
    if map.contains_key(&index)
    {
        map.remove(&index);
    }
}