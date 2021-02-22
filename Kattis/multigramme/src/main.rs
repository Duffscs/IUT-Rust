use std::io;
use std::io::Read;
fn is_equal(texte : Vec<char>, milieu : i32) -> bool
{
    let mut texte1 : Vec<char>= Vec::new();
    let mut texte2 : Vec<char> = Vec::new();
    for i in 0..milieu
    {
        texte1.push(texte[i as usize]);
        texte2.push(texte[(i+milieu) as usize]);

    }
    if texte1.eq(texte2);


    return true;
}
fn main() 
{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let words = input;
    let vector: Vec<char>= words.chars().collect();
    if vector.len() % 2 == 0
    {

    }
    else
    {
        println!("-1");
    }

}
