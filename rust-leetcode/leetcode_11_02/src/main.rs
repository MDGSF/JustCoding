impl Solution {
  pub fn max_area(height: Vec<i32>) -> i32 {
    let mut result = i32::min_value();
    for i in 0..(height.len() - 1) {
      for j in (i + 1)..(height.len()) {
        result = i32::max(result, i32::min(height[i], height[j]) * (j - i) as i32);
      }
    }
    result
  }
}

struct Solution {}

fn main() {
  let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
  let result = Solution::max_area(height);
  println!("result = {}", result);
}
