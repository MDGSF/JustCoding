use leetcode_11::solution1;
use leetcode_11::solution2;

fn main() {
  let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
  let result = solution2::Solution::max_area(height);
  println!("result = {}", result);
}
