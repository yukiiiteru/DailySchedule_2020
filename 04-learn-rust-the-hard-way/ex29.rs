fn main() {
    let people = 20;
    let cats = 30;
    let dogs = 15;

    if people < cats {
        println!("Too many cats! The world is doomed!");
    }

    if people > cats {
        println!("Not many cats! The world is saved!");
    }

    if people < dogs {
        println!("The world is drooled on!");
    }

    if people > dogs {
        println!("The world is dry");
    }

    let dogs = dogs + 5;

    if people >= dogs {
        println!("People are greater than or equal to dogs.");
    }

    if people <= dogs {
        println!("People are less than or equal to dogs.");
    }
    
    if people == dogs {
        println!("People are dogs.");
        // ??????
    }
}
