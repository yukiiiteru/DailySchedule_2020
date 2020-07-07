fn main() {
    let my_name = "Zed A. Shaw";
    let my_age = 35;
    let my_height = 74;
    let my_weight = 180;
    let my_eyes = "Blue";
    let my_teeth = "White";
    let my_hair = "Brown";

    let height_cm = (my_height as f64) * 2.54;
    let weight_kg = (my_weight as f64) * 0.45359237;

    println!("Let's talk about {}.", my_name);
    println!("He's {} inches tall.", my_height);
    println!("He's {} pounds heavy.", my_weight);
    println!("Actually that's not too heavy.");
    println!("He's got {} eyes and {} hair.", my_eyes, my_hair);
    println!("His teeth are usually {} depending on the coffee.", my_teeth);

    println!("If I had {}, {}, and {} I get {}",
            my_age, my_height, my_weight,
            my_age + my_height + my_weight);

    println!("He's {} cm tall", height_cm);
    println!("He's {} kg heavy.", weight_kg);
}
