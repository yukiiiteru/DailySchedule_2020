fn main() {
    println!("I will now count my chickens:");
    
    println!("Hens {}", 25 + 30 / 6);
    println!("Roosters {}", 100 - 25 * 3 % 4);

    println!("Now I will count the eggs");

    println!("{}", 3 + 2 + 1 - 5 + 4 % 2 - 1 / 4 + 6);

    println!("Is it true that 3 + 2 < 5 - 7?");

    println!("{}", 3 + 2 < 5 - 7);

    println!("What is 3 + 2? {}", 3 + 2);
    println!("What is 5 - 7? {}", 5 - 7);

    println!("Oh, that's why it's false");

    println!("How about some more.");

    println!("Is it greater? {}", 5 > -2);
    println!("Is it greater or equal? {}", 5 >= -2);
    println!("Is it less or equal? {}", 5 <= -2);
}
