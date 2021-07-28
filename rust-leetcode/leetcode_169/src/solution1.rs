impl Solution {
  pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut result = nums[0];
    let mut count = 1;
    for i in 1..nums.len() {
      if result == nums[i] {
        count += 1;
      } else {
        if count == 1 {
          result = nums[i];
        } else {
          count -= 1;
        }
      }
    }
    result
  }
}

pub struct Solution;
