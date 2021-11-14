impl Solution {
  pub fn hammingWeight(mut n: u32) -> i32 {
    n.count_ones() as i32
  }
}

pub struct Solution;
