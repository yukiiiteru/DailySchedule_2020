struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
       let mut v: Vec<char> = Vec::new();
       for ch in s.chars() {
           if ch == '(' || ch == '[' || ch == '{' {
               v.push(ch);
           } else {
               let c = match v.pop() {
                   Some(c)  => c,
                   None     => return false,
               };
               if !(c == '(' && ch == ')' ||
                  c == '[' && ch == ']' ||
                  c == '{' && ch == '}') {
                    return false;
               }
           }
       }
       v.len() == 0
    }
}

fn main() {
    println!("{}", Solution::is_valid("()".to_string()));
    println!("{}", Solution::is_valid("()[]{}".to_string()));
    println!("{}", Solution::is_valid("(]".to_string()));
    println!("{}", Solution::is_valid("([)]".to_string()));
    println!("{}", Solution::is_valid("{[]}".to_string()));
}
