fn main() {
    let x = format!("There are {} types of people.", 10);
    let binary = "binary";
    let do_not = "don't";
    let y = format!("Those who know {} and those who {}.", binary, do_not);

    println!("{}", x);
    println!("{}", y);

    println!("I said: {:?}.", x);
    println!("I also said: '{}'.", y);

    let hilarious = false;
    let joke_evaluation = format!("Isn't that joke so funny?! {:?}", hilarious);

    println!("{}", joke_evaluation);

    let w = String::from("This is the left side of...");
    let e = "a string with a right side.";
    println!("{}", w + e);
}
