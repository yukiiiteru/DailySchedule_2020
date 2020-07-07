fn main() {
    let days = "Mon Tue Wed Thu Fri Sat Sun";
    let months = "Jan\nFeb\nMar\nApr\nMay\nJun\nJul\nAug";

    println!("Here are the days: {}", days);
    println!("Here are the months: {}", months);

    // Rust doesn't support this grammar
    //
    // println!("""
    // There's something going on here.
    // With the three double-quotes.
    // We'll be able to type as much as we like.
    // Even 4 lines if we want, or 5, or 6.
    // """);
}
