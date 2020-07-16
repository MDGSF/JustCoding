impl Solution {
  pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort();

    let mut result = vec![];

    for i in 0..nums.len() {
      if i > 0 && nums[i] == nums[i - 1] {
        continue;
      }

      let target = -nums[i];
      let mut left = i + 1;
      let mut right = nums.len() - 1;
      while left < right {
        let sum = nums[left] + nums[right];
        if sum < target {
          left += 1;
        } else if sum > target {
          right -= 1;
        } else {
          result.push(vec![nums[i], nums[left], nums[right]]);

          while left + 1 < nums.len() && nums[left] == nums[left + 1] {
            left += 1;
          }
          left += 1;

          while right >= 1 && nums[right] == nums[right - 1] {
            right -= 1;
          }
          if right > 0 {
            right -= 1;
          }
        }
      }
    }

    result
  }
}

struct Solution;

fn main() {
  //let nums = vec![-1, 0, 1, 2, -1, 4];
  let nums = vec![0, 0, 0];
  let result = Solution::three_sum(nums);
  println!("result = {:?}", result);
}
