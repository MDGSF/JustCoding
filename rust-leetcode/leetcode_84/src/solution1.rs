impl Solution {
  pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut max_area = 0;
    for i in 0..heights.len() {
      for j in i..heights.len() {
        let mut min_height = heights[i];
        for k in i..=j {
          min_height = min_height.min(heights[k]);
        }
        let cur_area = min_height * (j - i + 1) as i32;
        max_area = max_area.max(cur_area);
      }
    }
    max_area
  }
}

pub struct Solution;
