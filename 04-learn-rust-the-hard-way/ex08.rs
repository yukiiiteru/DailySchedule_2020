fn main() {
    let formatter = "{:?} {:?} {:?} {:?}";

    println!("{:?} {:?} {:?} {:?}", 1, 2, 3, 4);
    println!("{:?} {:?} {:?} {:?}", "one", "two", "three", "four");
    println!("{:?} {:?} {:?} {:?}", true, false, false, true);
    println!("{:?} {:?} {:?} {:?}", formatter, formatter, formatter, formatter);
    println!("{:?} {:?} {:?} {:?}",
            "I had this thing.",
            "That you could type up right.",
            "But it didn't sing",
            "So I said goodnight.");
}
