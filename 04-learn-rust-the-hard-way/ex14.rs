use std::env;
use std::io;
use std::io::Write;

fn readln(s: &mut String) {
    io::stdin().read_line(s).unwrap();
    *s = s.trim().to_string();
}

fn print_prompt() {
    print!("> ");
    io::stdout().flush();
}

fn main() {
    let mut args = env::args();
    let script = args.next().unwrap();
    let user_name = match args.next() {
        Some(s) => s,
        None    => String::from(""),
    };

    println!("Hi {}, I'm the {} script.", user_name, script);
    println!("I'd like to ask you a few queserions.");
    println!("Do you like me {}?", user_name);
    print_prompt();
    let mut likes = String::new();
    readln(&mut likes);

    println!("Where do you live {}?", user_name);
    print_prompt();
    let mut lives = String::new();
    readln(&mut lives);

    println!("Where kind of computer do you have?");
    print_prompt();
    let mut computer = String::new();
    readln(&mut computer);

    println!("Alright, so you said {:?} about liking me.", likes);
    println!("You live in {:?}. Not sure where that is.", lives);
    println!("And you have a {:?} computer. Nice.", computer);
}

