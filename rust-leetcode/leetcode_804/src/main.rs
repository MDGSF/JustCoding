use leetcode_804::solution1::Solution;

fn main() {
  let words = vec![
    "gin".to_string(),
    "zen".to_string(),
    "gig".to_string(),
    "msg".to_string(),
  ];
  let result = Solution::unique_morse_representations(words);
  println!("result = {}", result);
}
