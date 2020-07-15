fn main() {
    let ten_things = "Apples Oranges Crows Telephone Light Sugar";
    
    println!("Wait there's not 10 things in that list, let's fix that.");

    let mut stuff = ten_things.split(' ').collect::<Vec<_>>();
    let mut more_stuff = vec!["Day", "Night", "Song", "Frisbee", "Corn", "Banana", "Girl", "Boy"];

    while stuff.len() != 10 {
        let next_one = more_stuff.pop().unwrap();
        println!("Adding: {}", next_one);
        stuff.push(next_one);
        println!("There's {} items now.", stuff.len());
    }

    println!("There we go: {:?}", stuff);

    println!("Let's do some things with stuff.");

    println!("{}", stuff[1]);
    println!("{}", stuff[stuff.len()-1]);
    println!("{}", stuff.pop().unwrap());
    println!("{}", stuff.join(" "));
    println!("{}", stuff[3..5].join("#"));
}
