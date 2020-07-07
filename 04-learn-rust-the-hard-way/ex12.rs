fn readln(s: &mut String) {
    std::io::stdin().read_line(s).unwrap();
    *s = s.trim().to_string();
}

fn main(){
    println!("How old are you?");
    let mut age = String::new();
    readln(&mut age);
    println!("How tall are you?");
    let mut height = String::new();
    readln(&mut height);
    println!("How much do you weigh?");
    let mut weight = String::new();
    readln(&mut weight);

    println!("So, you're {} old, {} tall and {} heavy.", age, height, weight);
}
