fn main(){
    println!("How old are you?");
    let mut age = String::new();
    std::io::stdin().read_line(&mut age).unwrap();
    age = age.trim().to_string();
    println!("How tall are you?");
    let mut height = String::new();
    std::io::stdin().read_line(&mut height).unwrap();
    height = height.trim().to_string();
    println!("How much do you weigh?");
    let mut weight = String::new();
    std::io::stdin().read_line(&mut weight).unwrap();
    weight = weight.trim().to_string();

    println!("So, you're {} old, {} tall and {} heavy.", age, height, weight);
}
