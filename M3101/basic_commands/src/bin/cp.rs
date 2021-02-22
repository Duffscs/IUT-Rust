use std::io::{self,BufRead, Read,Result};
use std::fs;
use std::process::exit;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    #[structopt()]
    fichier1: String,
    fichier2 : String
}

fn main() -> Result<()> {
    let options = Options::from_args();
    fs::copy(options.fichier1,options.fichier2)?;
    Ok(())
}