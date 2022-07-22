use std::io::{self, BufRead};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    #[structopt()]
    pattern: String
}

fn main() {
    let options = Options::from_args();
    io::stdin().lock().lines()
        .map(|line|
            line.expect("line")
        ).filter(|line| {
            return line.contains((&options.pattern));
        }).for_each(|line| {
            println!("{}", line);
        });
}
