impl Solution {
  pub fn climb_stairs(n: i32) -> i32 {
    if n < 3 {
      return n;
    }
    let mut f1 = 1;
    let mut f2 = 2;
    for _ in 2..n {
      let next = f1 + f2;
      f1 = f2;
      f2 = next;
    }
    return f2;
  }
}

struct Solution;

fn main() {
  let result = Solution::climb_stairs(4);
  println!("result = {}", result);
}
