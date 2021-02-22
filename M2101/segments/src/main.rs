const NB_ITERATIONS: i32 = 3;
static GLOBAL_STATIC_VAR: i32 = 99;

struct Point {
    x: f64,
    y: f64,
}

fn play_with_variables() {
    let local_var = 5;
    static LOCAL_STATIC_VAR: i32 = -33;
    for i in 0..NB_ITERATIONS {
        println!("Variables: {} {} {} {}", i, GLOBAL_STATIC_VAR, LOCAL_STATIC_VAR, local_var);
    }
    play_with_structs();
}

fn play_with_structs() {
    let point = Point{x: -2.0, y: 3.5};
    println!("Point: {} {}", point.x, point.y);
    play_with_arrays();
}

fn play_with_arrays() {
    let array = [15, 16, 17, 18];
    let mut vector = vec![18, 19, 20, 21];
    vector.push(22);
    println!("Arrays: {:?} {:?}", array, vector);
    play_with_strings();
}

fn play_with_strings() {
    let hello = "Bonjour";
    let mut world = String::from("Limoges");
    world.push_str(" !");
    println!("Strings: {} {}", hello, world);
}

fn main() {
    play_with_variables();
}
