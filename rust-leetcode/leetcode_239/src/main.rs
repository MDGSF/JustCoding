use leetcode_239::solution2::Solution;

fn main() {
  let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
  let k = 3;
  let result = Solution::max_sliding_window(nums, k);
  println!("result = {:?}", result);
}
