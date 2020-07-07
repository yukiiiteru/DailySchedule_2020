use std::env;
use std::io;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn readln(s: &mut String) {
    match io::stdin().read_line(s) {
        Ok(_)   => *s = s.trim().to_string(),
        Err(_)  => *s = String::from(""),
    }
}

fn print(s: &str) {
    print!("{}", s);
    io::stdout().flush().unwrap();
}

fn main() {
    let mut args = env::args();
    args.next().unwrap();
    let filename = match args.next() {
        Some(s) => s,
        None    => panic!("No input file"),
    };

    println!("We're going to erase {:?}.", filename);
    println!("If you don't want that, hiit CTRL-C (^C).");
    println!("If you do want that, hit RETURN.");

    {
        print("?");
        let mut null = String::new();
        readln(&mut null);
    }

    println!("Opening the file...");
    let path = Path::new(&filename);
    println!("Here's your file: {:?}", filename);
    let mut target = File::create(&path).unwrap();

    println!("Now I'm going to ask you for three lines.");

    print("line 1: ");
    let mut line1 = String::new();
    readln(&mut line1);
    print("line 2: ");
    let mut line2 = String::new();
    readln(&mut line2);
    print("line 3: ");
    let mut line3 = String::new();
    readln(&mut line3);

    println!("I'm going to write these to the file.");

    target.write(line1.as_bytes()).unwrap();
    target.write("\n".as_bytes()).unwrap();
    target.write(line2.as_bytes()).unwrap();
    target.write("\n".as_bytes()).unwrap();
    target.write(line3.as_bytes()).unwrap();
    target.write("\n".as_bytes()).unwrap();

    println!("And finally, we close it.");
}
