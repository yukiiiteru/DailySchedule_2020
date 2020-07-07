struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut hashmap: HashMap<i32, i32> = HashMap::new();
        for i in 0..nums.len() {
            let n = nums[i];
            let sub = target - n;
            if let Some(res) = hashmap.get(&sub) {
                return vec![*res, i as i32];
            }
            hashmap.insert(n, i as i32);
        }
        Vec::<i32>::new()
    }
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let res = Solution::two_sum(nums, target);
    println!("{:?}", res);
}
