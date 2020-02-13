impl Solution {
  pub fn max_area(height: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = height.len() - 1;
    let mut result = 0;
    while left < right {
      let cur_height = std::cmp::min(height[left], height[right]);
      let cur_width = (right - left) as i32;
      let cur_area = cur_height * cur_width;
      result = std::cmp::max(result, cur_area);
      if height[left] < height[right] {
        left += 1;
      } else {
        right -= 1;
      }
    }
    result
  }
}

struct Solution {}

fn main() {
  println!("Hello, world!");
  let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
  let result = Solution::max_area(height);
  println!("result = {}", result);
}
