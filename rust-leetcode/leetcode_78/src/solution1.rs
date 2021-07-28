impl Solution {
  pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut cur = Vec::new();
    let mut result = Vec::new();
    Solution::recursion(&nums, 0, &mut cur, &mut result);
    result
  }

  fn recursion(nums: &[i32], first: usize, cur: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    result.push(cur.clone());
    for i in first..nums.len() {
      cur.push(nums[i]);
      Solution::recursion(&nums, i + 1, cur, result);
      cur.pop();
    }
  }
}

pub struct Solution;
