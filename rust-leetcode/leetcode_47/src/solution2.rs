impl Solution {
  pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    let mut cur = Vec::new();
    let mut result = Vec::new();
    let mut used = vec![false; nums.len()];
    Solution::recursion(&nums, &mut used, &mut cur, &mut result);
    result
  }

  fn recursion(
    nums: &Vec<i32>,
    used: &mut Vec<bool>,
    cur: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
  ) {
    if nums.len() == cur.len() {
      result.push(cur.clone());
      return;
    }

    for i in 0..nums.len() {
      if used[i] {
        continue;
      }
      if i > 0 && nums[i - 1] == nums[i] && !used[i - 1] {
        continue; // 去掉重复的数字
      }

      used[i] = true;
      cur.push(nums[i]);

      Solution::recursion(nums, used, cur, result);

      used[i] = false;
      cur.pop();
    }
  }
}

pub struct Solution;
