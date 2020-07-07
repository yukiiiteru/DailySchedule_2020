fn main() {
    let cars = 100;
    let space_in_a_car = 4.0;
    let drivers = 30;
    let passengers = 90;
    let cars_not_driven = cars - drivers;
    let cars_driven = drivers;
    let carpool_capacity = (cars_driven as f64) * space_in_a_car;
    let average_passengers_per_car = passengers / cars_driven;

    println!("There are {} cars available.", cars);
    println!("There are only {} drivers available.", drivers);
    println!("There will be {} empty cars today.", cars_not_driven);
    println!("We can transport {} people today.", carpool_capacity);
    println!("We have {} to carpool today.", passengers);
    println!("We need to put about {} in each car.", average_passengers_per_car);
}
