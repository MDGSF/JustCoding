use leetcode_1309::solution1::Solution;

fn main() {
  let s = "10#11#12".to_string();
  let result = Solution::freq_alphabets(s);
  println!("result = {}", result);
}
