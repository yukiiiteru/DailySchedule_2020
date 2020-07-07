struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 { return String::from(""); }
        let mut min_len = std::usize::MAX;
        for s in &strs {
            if s.len() < min_len {
                min_len = s.len();
            }
        }
        for i in 0..min_len {
            let first_ch = &strs[0].as_bytes()[i];
            for j in 1..strs.len() {
                let ch = &strs[j].as_bytes()[i];
                if first_ch != ch {
                    return String::from(&strs[0][0..i]);
                }
            }
        }
        String::from(&strs[0][0..min_len])
    }
}

fn main() {
    let strs1 = vec![String::from("flower"),
                    String::from("flow"), 
                    String::from("flight")];
    let strs2 = vec![String::from("dog"),
                    String::from("racecar"), 
                    String::from("car")];
    let strs3 = vec![];
    let strs4 = vec![String::from("")];
    println!("{}", Solution::longest_common_prefix(strs1));
    println!("{}", Solution::longest_common_prefix(strs2));
    println!("{}", Solution::longest_common_prefix(strs3));
    println!("{}", Solution::longest_common_prefix(strs4));
}
