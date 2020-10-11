impl Solution {
  // 暴力法，一共两个子集，每个元素只有两种可能性，
  // 所以 n 个元素一共 2 ^ n 中可能性。
  pub fn can_partition_1(nums: Vec<i32>) -> bool {
    Solution::recursion(&nums, 0, 0, 0)
  }

  fn recursion(nums: &[i32], idx: usize, left: i32, right: i32) -> bool {
    if idx == nums.len() {
      return left == right;
    }
    return Solution::recursion(nums, idx + 1, left + nums[idx], right)
      || Solution::recursion(nums, idx + 1, left, right + nums[idx]);
  }

  // 动态规划
  // 问题转换：判断是否可以从数组中选出一些数字，使得这些数字的和等于数组
  // 的元素总和的一半。
  //
  // 数组长度为 n。
  // 数组所有元素总和 sum。
  // 数组中最大元素 maxNum。
  // target = sum / 2，也就是数组总和的一半。
  //
  // 创建二维数组 n 行 target + 1 列。
  // dp[i][j] 表示从数组的 [0, i] 下标范围内选取若干个元素（可以是 0 个），
  // 是否存在一个方案使得被选取的正整数的和等于 j。
  //
  // dp[i][j] = dp[i-1][j] | dp[i-1][j-numsi]], j >= nums[i]
  // dp[i][j] = dp[i-1][j]                    , j < nums[i]
  pub fn can_partition_2(nums: Vec<i32>) -> bool {
    let n = nums.len();
    if n < 2 {
      return false;
    }
    let mut sum = 0;
    let mut max_num = 0;
    for &num in nums.iter() {
      sum += num;
      max_num = max_num.max(num);
    }
    if sum % 2 != 0 {
      return false;
    }
    let target = sum / 2;
    if max_num > target {
      return false;
    }

    let mut dp = vec![vec![false; (target + 1) as usize]; n];
    for i in 0..n {
      dp[i][0] = true;
    }
    dp[0][nums[0] as usize] = true;
    for i in 1..n {
      let num = nums[i] as usize;
      for j in 1..=(target as usize) {
        if j >= num {
          dp[i][j] = dp[i - 1][j] | dp[i - 1][j - num];
        } else {
          dp[i][j] = dp[i - 1][j];
        }
      }
    }
    dp[n - 1][target as usize]
  }

  // 动态规划：一维数组
  pub fn can_partition(nums: Vec<i32>) -> bool {
    let n = nums.len();
    if n < 2 {
      return false;
    }
    let mut sum = 0;
    let mut max_num = 0;
    for &num in nums.iter() {
      sum += num;
      max_num = max_num.max(num);
    }
    if sum % 2 != 0 {
      return false;
    }
    let target = sum / 2;
    if max_num > target {
      return false;
    }

    let mut dp = vec![false; (target + 1) as usize];
    dp[0] = true;
    for i in 1..n {
      let num = nums[i] as usize;
      for j in (1..=(target as usize)).rev() {
        if j >= num {
          dp[j] = dp[j] | dp[j - num];
        }
      }
    }
    dp[target as usize]
  }
}

struct Solution;

fn main() {
  //let nums = vec![1, 5, 11, 5];
  //let nums = vec![1, 2, 3, 5];
  let nums = vec![2, 2, 3, 5];
  let result = Solution::can_partition(nums);
  println!("result = {}", result);
}
