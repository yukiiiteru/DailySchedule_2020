use std::env;
use std::io;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::io::prelude::*;

fn readln(s: &mut String) {
    io::stdin().read_line(s).unwrap();
    *s = s.trim().to_string();
}

fn print_prompt() {
    print!("> ");
    io::stdout().flush().unwrap();
}

fn main() {
    let mut args = env::args();
    args.next().unwrap();
    let filename = match args.next() {
        Some(s) => s,
        None    => panic!("No input file"),
    };

    let path = Path::new(&filename);

    println!("Here's your file: {:?}", filename);
    let mut file = File::open(&path).unwrap();
    let mut txt = String::new();
    file.read_to_string(&mut txt).unwrap();
    println!("{}", txt);

    println!("Type the filename again:");
    print_prompt();
    let mut filename_again = String::new();
    readln(&mut filename_again);

    let path_again = Path::new(&filename_again);
    let mut file_again = File::open(&path_again).unwrap();

    let mut txt_again = String::new();
    file_again.read_to_string(&mut txt_again).unwrap();
    
    println!("{}", txt_again);
}
