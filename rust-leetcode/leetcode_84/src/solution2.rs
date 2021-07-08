impl Solution {
  pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut max_area = 0;
    let mut stack: Vec<i32> = vec![-1];
    for i in 0..heights.len() {
      while *stack.last().unwrap() != -1 && heights[i] <= heights[*stack.last().unwrap() as usize] {
        let top = stack.pop().unwrap() as usize;
        let cur_area = heights[top] * (i as i32 - stack.last().unwrap() - 1);
        max_area = max_area.max(cur_area);
      }
      stack.push(i as i32);
    }
    while *stack.last().unwrap() != -1 {
      let top = stack.pop().unwrap() as usize;
      let cur_area = heights[top] * (heights.len() as i32 - stack.last().unwrap() - 1);
      max_area = max_area.max(cur_area);
    }
    max_area
  }
}

pub struct Solution;
