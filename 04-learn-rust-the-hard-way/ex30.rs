fn main() {
    let people = 30;
    let cars = 40;
    let buses = 15;

    if cars > people {
        println!("We should take the cars.");
    } else if cars < people {
        println!("We should not take the cars.");
    } else {
        println!("We can't decide.");
    }

    if buses > cars {
        println!("That's too many buses.");
    } else if buses < cars {
        println!("Maybe we could take the buses.");
    } else {
        println!("We still can't decide.");
    }

    if people > buses {
        println!("Alright, let's just take the buses.");
    } else {
        println!("Fine, let's stay home then.");
    }
}
