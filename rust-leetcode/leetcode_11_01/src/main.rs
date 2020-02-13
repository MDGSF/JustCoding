impl Solution {
  pub fn max_area(height: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut i = 0;
    while i < height.len() - 1 {
      let mut j = i + 1;
      while j < height.len() {
        let cur_height = std::cmp::min(height[i], height[j]);
        let cur_width = (j - i) as i32;
        let cur_area = cur_height * cur_width;
        result = std::cmp::max(result, cur_area);
        j += 1;
      }
      i += 1;
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
