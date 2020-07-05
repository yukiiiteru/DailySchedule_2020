fn give_commoner(gift: Option<&str>) {
    match gift {
        Some("snake")   => println!("Yuck! I'm throwing that snake in a fire."),
        Some(inner)     => println!("{}? How nice.", inner),
        None            => println!("No gift? Oh well."),
    }
}

fn give_princess(gift: Option<&str>) {
    let inside = gift.unwrap();
    if inside == "snake" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s!!!!!", inside);
}

fn main() {
    let food  = Some("chicken");
    let snake = Some("snake");
    let void  = None;

    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

    let bird = Some("robin");
    let nothing = None;

    give_princess(bird);
    give_princess(nothing);
}

