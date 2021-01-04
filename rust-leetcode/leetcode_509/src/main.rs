use leetcode_509::solution5::Solution;

fn main() {
  for n in 0..=30 {
    let result = Solution::fib(n);
    print!("{}, ", result);
  }
}
