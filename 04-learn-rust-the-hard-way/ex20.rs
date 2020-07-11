use std::env;
use std::fs::File;
use std::io::SeekFrom;
use std::io::prelude::*;
use std::io::{ BufRead, BufReader };

fn print_all(f: &mut File) {
    let mut res = String::new();
    f.read_to_string(&mut res).unwrap();
    println!("{}", res);
}

fn rewind(f: &mut File) {
    f.seek(SeekFrom::Start(0)).unwrap();
}

fn print_a_line(line_count:i32, br: &mut dyn BufRead) {
    let mut res = String::new();
    br.read_line(&mut res).unwrap();
    println!("{} {}", line_count, res);
}

fn main() {
    let mut args = env::args();
    args.next().unwrap();
    let input_file = match args.next() {
        Some(s) => s,
        None    => panic!("No input file"),
    };

    let mut current_file = File::open(input_file).unwrap();

    println!("First let's print the whole file:\n");

    print_all(&mut current_file);

    println!("Now let's rewind, kind of like a tape.");

    rewind(&mut current_file);

    println!("Let's print three lines:");
    let mut buf_reader = BufReader::new(&mut current_file);

    let mut current_line = 1;
    print_a_line(current_line, &mut buf_reader);

    current_line += 1;
    print_a_line(current_line, &mut buf_reader);

    current_line += 1;
    print_a_line(current_line, &mut buf_reader);
}

