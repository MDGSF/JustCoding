impl Solution {
  pub fn hammingWeight(mut n: u32) -> i32 {
    let mut count = 0;
    while n != 0 {
      count += n & 1;
      n = n >> 1;
    }
    count as i32
  }
}

pub struct Solution;
