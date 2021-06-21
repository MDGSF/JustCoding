use std::collections::HashMap;

impl Solution {
  pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
      map.insert(num, i as i32);
    }

    for (i, &num) in nums.iter().enumerate() {
      let peer = target - num;
      if map.contains_key(&peer) && map.get(&peer) != Some(&(i as i32)) {
        return vec![*map.get(&peer).unwrap(), i as i32];
      }
    }

    vec![]
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
  }
}
