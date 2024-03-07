use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut m = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            if let Some(&j) = m.get(&(target - num)) {
                return vec![j as i32, i as i32];
            }
            m.insert(num, i);
        }
        return vec![];
    }
}
fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    println!("{:?}", result);
}
