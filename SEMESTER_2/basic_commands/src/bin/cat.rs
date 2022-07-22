use std::io::{self, BufRead, Read, Result, copy};
use std::process::exit;
use structopt::StructOpt;
use std::fs::File;
use std::str;

#[derive(StructOpt)]
struct Options {
    #[structopt()]
    arg: Vec<String>,
}

fn main() -> Result<()> {
    let options = Options::from_args();
    if options.arg.is_empty() {
        std::io::copy(&mut io::stdin(), &mut io::stdout());
    } else {
        for i in options.arg {
            println!("Fichier {}:",i);
            let mut fichier = File::open(i)?;
            let mut contenu: Vec<u8> = vec![];
            fichier.read_to_end(&mut contenu)?;
            let s = str::from_utf8(&*contenu).expect("conversion en utf8");
            println!("{}\n", s);
        }
    }
    Ok(())
}