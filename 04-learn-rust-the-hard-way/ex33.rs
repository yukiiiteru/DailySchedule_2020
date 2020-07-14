fn main() {
    let mut i = 0;
    let mut numbers: Vec<i32> = Vec::new();
    while i < 6 {
        println!("At the top i is {}", i);
        numbers.push(i);

        i = i + 1;
        println!("Numbers now: {:?}", numbers);
        println!("At the bottom i is {}", i);
    }
    
    println!("The numbers: ");

    for num in numbers {
        println!("{}", num);
    }
}
