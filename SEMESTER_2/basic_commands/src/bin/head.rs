use std::io::{self, BufRead};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    #[structopt()]
    stop: usize
}

fn main() {
    let options = Options::from_args();
    io::stdin().lock().lines()
        .map(|line|
            line.expect("line")
        ).take(options.stop)
        .for_each(|line| {
            println!("{}", line);
        });
}
