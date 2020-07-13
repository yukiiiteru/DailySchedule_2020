use std::io;
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
    println!("You enter a dark room with two doors.  Do you go through door #1 or door #2?");
    print_prompt();
    let mut door = String::new();
    readln(&mut door);

    if door == "1" {
        println!("There's a giant bear here eating a cheese cake.  What do you do?");
        println!("1. Take the cake.");
        println!("2. Scream at the bear.");
        print_prompt();
        let mut bear = String::new();
        readln(&mut bear);

        if bear == "1" {
            println!("The bear eats your face off.  Good job!");
        } else if bear == "2" {
            println!("The bear eats your legs off.  Good job!");
        } else {
            println!("Well, doing {} is probably better. Bear runs away.", bear);
        }
    } else if door == "2" {
        println!("You stare into the endless abyss at Cthulhu's retina.");
        println!("1. Blueberries.");
        println!("2. Yellow jacket clothespins.");
        println!("3. Understanding revolvers yelling melodies.");

        print_prompt();
        let mut insanity = String::new();
        readln(&mut insanity);

        if insanity == "1" || insanity == "2" {
            println!("Your body survives powered by a mind of jello.  Good job!");
        } else {
            println!("The insanity rots your eyes into a pool of muck.  Good job!");
        }
    } else {
        println!("You stumble around and fall on a knife and die.  Good job!");
    }
}
