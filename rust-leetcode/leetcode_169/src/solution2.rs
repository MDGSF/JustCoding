impl Solution {
  pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut vote = 0;
    for &num in nums.iter() {
      if vote == 0 {
        result = num;
      }

      if result == num {
        vote += 1;
      } else {
        vote -= 1;
      }
    }
    result
  }
}

pub struct Solution;
