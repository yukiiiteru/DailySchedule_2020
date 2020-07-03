fn main() {
    let fmt = format!("{} days", 31);
    println!("{}", fmt);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}", 
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    println!("{number:>width$}", number=1, width=6);

    println!("{number:>0width$}", number=1, width=6);

    println!("Just for test: {:>6}", 1);
    println!("Just for test: {:06}", 1);

    println!("My name is {0}, {1} {0}", "Bond", "James");

    #[derive(Debug)]
    struct Structure(i32);

    // println!("This struct `{}` won't print...", Structure(3));
    println!("But this can: {:?}", Structure(3));

    // These code is from docs
    println!("Hello, {:<5}", "x");
    println!("Hello, {:-<5}", "x");
    println!("Hello, {:^5}", "x");
    println!("Hello, {:>5}", "x");

    println!("Hello {0} is {1:.5}", "x", 0.01);

    println!("Hello {1} is {2:.0$}", 5, "x", 0.01);

    println!("Hello {0} is {2:.1$}", "x", 5, 0.01);

    println!("Hello {} is {:.*}",    "x", 5, 0.01);

    println!("Hello {} is {2:.*}",   "x", 5, 0.01);

    println!("Hello {} is {number:.prec$}", "x", prec = 5, number = 0.01);

    // rust-by-example
    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);
}

