fn main() {
    println!("la moyenne est {}",average(10.0,10.0));
    println!("la moyenne est {}",average(10.0,10.0));
    let rec= Rectangle{
        length:12.0,
        width:5.0,
    };

    println!("la perimetre est {}",perimeter2(&rec));
    println!("la perimetre est {}",perimeter(rec));
    println!("la perimetre est {}",perimeter(rec));

    let mut my_vec=vec![10.0,12.2];
    println!("la somme est {}", sum(&my_vec));
    println!("la somme est {}", sum(&my_vec));
    let mut x=10.0;
    let mut y=11.0;
    swap(& mut x,& mut y);
    swap(& mut x,& mut y);
    println!("{} {}",x,y);

    let mut  rec2= Rectangle{
        length:12.0,
        width:5.0,
    };
    scale(& mut rec2,10.0);
    scale(& mut rec2,10.0);
    println!("{} {}",rec2.length,rec2.width);

    cancel_out(&mut my_vec);
    cancel_out(&mut my_vec);
    println!("{} {}",my_vec[0],my_vec[1]);
    
}

fn swap( x : &mut f64, y: &mut f64)
{
    let buff=*x;
    *x=*y;
    *y=buff;

}

fn scale(rec : &mut Rectangle,y: f64)
{
    rec.length=rec.length*y;
    rec.width=rec.width*y;
    
}

fn cancel_out(vec : &mut Vec<f64>)
{
   for i in 0..vec.len()
   {
       vec[i]=0.0;
   }
}


fn sum(tab : &Vec<f64>)->f64
{
    let mut som=0.0;
    for i in 0..tab.len() 
    {//emprunt 
        som+=tab[i];
    } 
    return som;
}


fn average(x : f64, y: f64) ->f64 {(x+y)/2.0}

#[derive(Copy,Clone)]
struct Rectangle
{
    length: f64,
    width: f64,
}

fn perimeter(rect : Rectangle)->f64{
    rect.length*2.0+rect.width*2.0
}

fn perimeter2(rect : &Rectangle)->f64{
    rect.length*2.0+rect.width*2.0
}



