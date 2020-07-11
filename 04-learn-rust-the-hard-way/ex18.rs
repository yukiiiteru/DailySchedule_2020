fn print_two(arg1: &str, arg2: &str) {
    println!("arg1: {:?}, arg2: {:?}", arg1, arg2);
}

fn print_two_again(arg1: &str, arg2: &str) {
    println!("arg1: {:?}, arg2: {:?}", arg1, arg2);
}

fn print_one(arg1: &str) {
    println!("arg1: {:?}", arg1);
}

fn print_none() {
    println!("I got nothing.");
}

fn main() {
    print_two("Zed", "Shaw");
    print_two_again("Zed", "Shaw");
    print_one("First!");
    print_none();
}
