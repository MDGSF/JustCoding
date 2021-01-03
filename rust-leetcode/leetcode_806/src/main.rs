use leetcode_806::solution1::Solution;

fn main() {
  let widths = vec![
    4, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
    10, 10,
  ];
  let s = "bbbcccdddaaa".to_string();
  let result = Solution::number_of_lines(widths, s);
  println!("result = {:?}", result);
}
