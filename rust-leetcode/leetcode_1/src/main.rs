use leetcode_1::solution1::Solution;

fn main() {
  let nums = vec![2, 7, 11, 15];
  let target = 9;
  let result = Solution::two_sum(nums, target);
  println!("result = {:?}", result);
}
