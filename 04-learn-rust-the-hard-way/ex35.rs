use std::io;
use std::process::exit;
use std::io::prelude::*;

fn readln(s: &mut String) {
    io::stdin().read_line(s).unwrap();
    *s = s.trim().to_string();
}

fn print_prompt() {
    print!("> ");
    io::stdout().flush().unwrap();
}

fn gold_room() {
    println!("This room is full of gold.  How much do you take?");

    print_prompt();
    let mut next = String::new();
    readln(&mut next);

    let mut how_much = 0;
    if let Ok(num) = next.parse::<u32>() {
        how_much = num;
    } else {
        dead("Man, learn to type a number.");
    }

    if how_much < 50 {
        println!("Nice, you're not greedy, you win!");
        exit(0);
    } else {
        dead("You greedy bastard!");
    }
}

fn bear_room() {
    println!("There is a bear here.");
    println!("The bear has a bunch of honey.");
    println!("The fat bear is in front of another door.");
    println!("How are you going to move the bear?");
    let mut bear_moved = false;

    loop {
        print_prompt();
        let mut next = String::new();
        readln(&mut next);

        if next == "take honey" {
            dead("The bear looks at you then slaps your face off.");
        } else if next == "taunt bear" && (! bear_moved) {
            println!("The bear has moved from the door. You can go through it now.");
            bear_moved = true;
        } else if next == "taunt bear" && bear_moved {
            dead("The bear gets pissed off and chews your leg off.");
        } else if next == "open door" && bear_moved {
            gold_room();
        } else {
            println!("I got no idea what that means.");
        }
    }
}

fn cthulhu_room() {
    println!("Here you see the great evil Cthulhu.");
    println!("He, it, whatever stares at you and you go insane.");
    println!("Do you flee for your life or eat your head?");

    print_prompt();
    let mut next = String::new();
    readln(&mut next);

    if next == "flee" {
        start();
    } else if next == "head" {
        dead("Well that was tasty!");
    } else {
        cthulhu_room();
    }
}

fn dead(why: &str) {
    println!("{}, Good job!", why);
    exit(0);
}

fn start() {
    println!("You are in a dark room.");
    println!("There is a door to your right and left.");
    println!("Which one do you take?");

    print_prompt();
    let mut next = String::new();
    readln(&mut next);

    if next == "left" {
        bear_room();
    } else if next == "right" {
        cthulhu_room();
    } else {
        dead("You stumble around the room until you starve.");
    }
}

fn main() {
    start();
}
