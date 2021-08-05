use leetcode_200::solution2::Solution;

fn main() {
  let grid = vec![
    vec!['1', '1', '1', '1', '0'],
    vec!['1', '1', '0', '1', '0'],
    vec!['1', '1', '0', '0', '0'],
    vec!['0', '0', '0', '0', '0'],
  ];
  let result = Solution::num_islands(grid);
  println!("result = {}", result);
}
