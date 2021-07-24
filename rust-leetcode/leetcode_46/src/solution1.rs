impl Solution {
  pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    Solution::recursion(&mut nums, 0, &mut result);
    result
  }

  fn recursion(nums: &mut [i32], first: usize, result: &mut Vec<Vec<i32>>) {
    if first == nums.len() {
      result.push(nums.to_vec());
      return;
    }

    let len = nums.len();
    for i in first..len {
      nums.swap(first, i);
      Solution::recursion(nums, first + 1, result);
      nums.swap(first, i);
    }
  }
}

pub struct Solution;
