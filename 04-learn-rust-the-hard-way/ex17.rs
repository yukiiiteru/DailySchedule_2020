use std::env;
use std::io;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn readln(s: &mut String) {
    match io::stdin().read_line(s) {
        Ok(_)   => *s = s.trim().to_string(),
        Err(_)  => *s = String::from(""),
    }
}

fn main() {
    let mut args = env::args();
    args.next().unwrap();
    let from_file = match args.next() {
        Some(s) => s,
        None    => panic!("No from file"),
    };
    let to_file = match args.next() {
        Some(s) => s,
        None    => panic!("No to file"),
    };

    let indata = fs::read(from_file).unwrap();

    println!("The input file is {} bytes long", indata.len());
    println!("Ready, hit RETURN to continue, CTRL-C to abort.");

    {
        let mut null = String::new();
        readln(&mut null);
    }

    let output_path = Path::new(&to_file);
    let mut output = File::create(&output_path).unwrap();
    output.write(&indata).unwrap();

    println!("Alright, all done.");
}
