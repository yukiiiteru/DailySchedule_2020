struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 { return false; }
        let mut num = x;
        let mut res: i32 = 0;
        loop {
            res = res * 10 + num % 10;
            num /= 10;
            if num == 0 { break; }
        }
        res == x
    }
}

fn main() {
    println!("{}", Solution::is_palindrome(121));
    println!("{}", Solution::is_palindrome(-121));
    println!("{}", Solution::is_palindrome(10));
}
