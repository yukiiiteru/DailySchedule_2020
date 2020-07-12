fn secret_formula(started: i32) -> (i32, i32, i32) {
    let jelly_beans = started * 500;
    let jars = jelly_beans / 1000;
    let crates = jars / 100;
    (jelly_beans, jars, crates)
}

fn main() {
    println!("Let's practice everything.");
    println!("You'd need to know 'bout escapes with \\ that do \n newlines and \t tabs.");

    let poem = "
\tThe lovely world
with logic so firmly planted
cannot discern \n the needs of love
nor comprehend passion from intuition
and requires an explanation
\n\t\twhere there is none.";
    
    println!("--------------");
    println!("{}", poem);
    println!("--------------");

    let five = 10 - 2 + 3 - 6;
    println!("This should be five: {}", five);

    let start_point = 10000;
    let (beans, jars, crates) = secret_formula(start_point);

    println!("With a starting point of: {}", start_point);
    println!("We'd have {} beans, {} jars, and {} crates.", beans, jars, crates);

    let start_point = start_point / 10;
    let (beans, jars, crates) = secret_formula(start_point);

    println!("We can also do that this way:");
    println!("We'd have {} beans, {} jars, and {} crates.", beans, jars, crates);
}
