use std::io;
use std::io::Read;

fn main() {
    let mut str = String::new();
    io::stdin().read_to_string(&mut str).expect("Lecture de stdin");
    str.lines().skip(1).for_each(|x| {
        let mut split = x.split(":");
        let hour: i32 = split.next().expect("str").parse().expect("i32");
        let minutes: i32 = split.next().expect("str").parse().expect("i32");
        let res = decal_hour(hour, minutes, -36);
        let res2 = decal_hour(hour, minutes, 36);
        print_hour(res.0, res.1);
        print!(" ");
        print_hour(res2.0, res2.1);
        println!();
    });
}

fn print_hour(hour: i32, minutes: i32) {
    if hour < 10 {
        print!("{}", 0);
    }
    print!("{}:", hour);
    if minutes < 10 {
        print!("{}", 0);
    }
    print!("{}", minutes);
}

fn decal_hour(hour: i32, minutes: i32, decal: i32) -> (i32, i32) {
    let mut decal_minutes = minutes + decal;
    let mut decal_hour = hour;
    if decal_minutes < 0 {
        decal_hour -= 1;
        decal_minutes += 60;
    } else if decal_minutes >= 60 {
        decal_hour += 1;
    }
    if decal_hour < 0 {
        decal_hour += 24;
    }
    decal_hour = decal_hour % 24;
    decal_minutes = decal_minutes % 60;
    return (decal_hour, decal_minutes);
}