use leetcode_1002::solution1::Solution;

fn main() {
  let a = vec![
    "bella".to_string(),
    "label".to_string(),
    "roller".to_string(),
  ];
  let result = Solution::common_chars(a);
  println!("result = {:?}", result);
}
