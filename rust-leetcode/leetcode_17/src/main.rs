use leetcode_17::solution1::Solution;

fn main() {
  let digits = "23".to_string();
  let result = Solution::letter_combinations(digits);
  println!("result = {:?}", result);
}
