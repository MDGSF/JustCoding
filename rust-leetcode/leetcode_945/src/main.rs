impl Solution {
  pub fn min_increment_for_unique(a: Vec<i32>) -> i32 {
    let mut count = vec![0; 80000];
    for x in a {
      count[x as usize] += 1;
    }
    let mut result: i32 = 0;
    let mut taken: i32 = 0;
    for x in 0..80000 {
      if count[x] >= 2 {
        taken += count[x] - 1;
        result -= (x as i32) * (count[x] - 1);
      } else if taken > 0 && count[x] == 0 {
        taken -= 1;
        result += x as i32;
      }
    }
    result
  }
}

struct Solution;

fn main() {
  let a = vec![1, 2, 2];
  let result = Solution::min_increment_for_unique(a);
  println!("result = {}", result);
}
