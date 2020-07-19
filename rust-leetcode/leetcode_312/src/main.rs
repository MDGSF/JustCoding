use std::collections::HashMap;

impl Solution {
  pub fn max_coins(nums: Vec<i32>) -> i32 {
    let mut new_nums = vec![1];
    new_nums.extend_from_slice(&nums);
    new_nums.push(1);
    let n = new_nums.len();
    let mut m = HashMap::new();
    Solution::dfs(&new_nums, &mut m, 0, n - 1)
  }

  fn dfs(nums: &[i32], mut m: &mut HashMap<(usize, usize), i32>, left: usize, right: usize) -> i32 {
    if left + 1 == right {
      return 0;
    }
    if m.contains_key(&(left, right)) {
      return *m.get(&(left, right)).unwrap();
    }
    let mut result = 0;
    for i in (left + 1)..right {
      let sub1 = Solution::dfs(nums, &mut m, left, i);
      let sub2 = Solution::dfs(nums, &mut m, i, right);
      let cur = nums[left] * nums[i] * nums[right] + sub1 + sub2;
      result = std::cmp::max(result, cur);
    }
    m.insert((left, right), result);
    result
  }
}

struct Solution;

fn main() {
  let nums = vec![3, 1, 5, 8];
  let result = Solution::max_coins(nums);
  println!("result = {}", result);
}
