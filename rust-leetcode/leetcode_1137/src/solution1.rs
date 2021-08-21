impl Solution {
  pub fn tribonacci(n: i32) -> i32 {
    if n == 0 {
      return 0;
    } else if n == 1 {
      return 1;
    } else if n == 2 {
      return 1;
    }
    let mut t0 = 0;
    let mut t1 = 1;
    let mut t2 = 1;
    for _ in 3..=n {
      let t = t0 + t1 + t2;
      t0 = t1;
      t1 = t2;
      t2 = t;
    }
    t2
  }
}

pub struct Solution;
