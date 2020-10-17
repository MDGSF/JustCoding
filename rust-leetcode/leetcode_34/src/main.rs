impl Solution {
  pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if let Some(left) = nums.iter().position(|&n| n == target) {
      if let Some(right) = nums.iter().rposition(|&n| n == target) {
        return vec![left as i32, right as i32];
      }
    }
    vec![-1, -1]
  }
}

struct Solution;

fn main() {
  let nums = vec![5, 7, 7, 8, 8, 10];
  let target = 8;
  //let nums = vec![1];
  //let target = 0;
  let result = Solution::search_range(nums, target);
  println!("result = {:?}", result);
}
