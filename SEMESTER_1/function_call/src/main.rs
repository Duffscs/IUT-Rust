fn to_minutes(hours: i32, minutes: i32) -> i32 {
	60*hours + minutes
}

fn main() 
{
	let h = 18;
	let m = 45;
	let total = to_minutes(h, m);
	println!("Total : {} minutes", total);
}
