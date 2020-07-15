use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn readln(s: &mut String) {
    io::stdin().read_line(s).unwrap();
    *s = s.trim().to_string();
}

fn print_prompt() {
    print!("> ");
    io::stdout().flush().unwrap();
}

fn find_city(themap: &HashMap<String, String>, state: &String) -> String {
    match themap.get(state) {
        Some(s) => s.to_string(),
        None    => "Not found.".to_string()
    }
}

fn main() {
    let mut cities = HashMap::new();
    cities.insert("CA".to_string(), "San Francisco".to_string());
    cities.insert("MI".to_string(), "Detroit".to_string());
    cities.insert("FL".to_string(), "Jacksonville".to_string());
    cities.insert("NY".to_string(), "New York".to_string());
    cities.insert("OR".to_string(), "Portland".to_string());

    loop {
        println!("State? (ENTER to quit)");
        print_prompt();
        let mut state = String::new();
        readln(&mut state);

        if state.len() == 0 {
            break;
        }

        let city_found = find_city(&cities, &state);
        println!("{}", city_found);
    }
}
