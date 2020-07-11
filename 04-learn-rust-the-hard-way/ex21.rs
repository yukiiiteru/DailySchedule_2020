fn add(a: i32, b: i32) -> i32 {
    println!("ADDING {} + {}", a, b);
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    println!("SUBTRACTING {} - {}", a, b);
    a - b
}

fn multiply(a: i32, b: i32) -> i32 {
    println!("MULTIPLYING {} * {}", a, b);
    a * b
}

fn divide(a: i32, b: i32) -> i32 {
    println!("DIVIDING {} / {}", a, b);
    a / b
}

fn main() {
    println!("Let's do some math with just functions!");

    let age = add(30, 5);
    let height = subtract(78, 4);
    let weight = multiply(90, 2);
    let iq = divide(100, 2);

    println!("Age: {}, Height: {}, Weight: {}, IQ: {}", age, height, weight, iq);

    println!("Here is a puzzle.");

    let what = add(age, subtract(height, multiply(weight, divide(iq, 2))));
    
    println!("That becomes: {} Can you do it by hand?", what);
}
