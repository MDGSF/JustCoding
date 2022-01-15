use leetcode_1036::solution1::Solution;

fn main() {
  //blocked = [[0, 3], [1, 0], [1, 1], [1, 2], [1, 3]]
  //source = [0, 0]
  //target = [0, 2]

  let blocked = vec![vec![0, 3], vec![1, 0], vec![1, 1], vec![1, 2], vec![1, 3]];
  let source = vec![0, 0];
  let target = vec![0, 2];
  let result = Solution::is_escape_possible(blocked, source, target);
  println!("result = {}", result);
}
