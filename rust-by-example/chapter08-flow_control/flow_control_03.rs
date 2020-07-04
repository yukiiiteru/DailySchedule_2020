fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            break 'outer;
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");
}
