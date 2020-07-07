struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut num = x;
        let mut res: i32 = 0;
        loop {
            res = match res.checked_mul(10) {
                Some(n) => n + num % 10,
                None    => return 0,
            };
            num /= 10;
            if num == 0 { break; }
        }
        res
    }
}

fn main() {
    println!("{}", Solution::reverse(123));
    println!("{}", Solution::reverse(-123));
    println!("{}", Solution::reverse(120));
    println!("{}", Solution::reverse(2147483647));
}
