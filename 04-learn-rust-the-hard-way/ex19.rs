fn chess_and_crackers(cheese_count: i32, boxes_of_crackers: i32) {
    println!("You have {} cheeses!", cheese_count);
    println!("You have {} boxes of crackers!", boxes_of_crackers);
    println!("May that's enough for a party!");
    println!("Get a blanket.\n");
}

fn main() {
    println!("We can just give the function numbers directly:");
    chess_and_crackers(20, 30);

    println!("OR, we can use variables from our script:");
    let amount_of_cheese = 10;
    let amount_of_crackers = 50;

    chess_and_crackers(amount_of_cheese, amount_of_crackers);

    println!("We can even do math inside too:");
    chess_and_crackers(10 + 20, 5 + 6);

    println!("And we can combine the two, variables and math:");
    chess_and_crackers(amount_of_cheese + 100, amount_of_crackers + 1000);
}
