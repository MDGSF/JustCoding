use std::collections::HashMap;

impl Solution {
  pub fn majority_element(nums: Vec<i32>) -> i32 {
    let m: HashMap<i32, i32> = nums.iter().fold(HashMap::new(), |mut acc, n| {
      *acc.entry(*n).or_insert(0) += 1;
      acc
    });

    let half = nums.len() as i32 / 2;
    for (&k, &v) in m.iter() {
      if v > half {
        return k;
      }
    }

    0
  }
}

pub struct Solution;
