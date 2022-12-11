use std::collections::HashMap;
use std::convert::TryInto;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut m = HashMap::new();
        for (idx, num) in nums.iter().enumerate() {
            let peer = target - num;
            if m.contains_key(&peer) {
                return vec![*m.get(&peer).unwrap(), idx.try_into().unwrap()];
            } else {
                m.insert(num, idx.try_into().unwrap());
            }
        }
        vec![]
    }
}

pub struct Solution;
