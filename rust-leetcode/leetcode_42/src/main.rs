impl Solution {
  pub fn trap(height: Vec<i32>) -> i32 {
    let mut result: i32 = 0;
    let mut stack: Vec<usize> = Vec::new();
    for i in 0..height.len() {
      while stack.len() > 0 && height[i] > height[*stack.last().unwrap()] {
        let tmp = stack.pop().unwrap();
        if stack.is_empty() {
          break;
        }
        let width: i32 = (i - stack.last().unwrap() - 1) as i32;
        let height: i32 = std::cmp::min(height[i], height[*stack.last().unwrap()]) - height[tmp];
        let area: i32 = width * height;
        result += area;
      }
      stack.push(i);
    }
    result
  }
}

struct Solution;

fn main() {
  let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
  let result = Solution::trap(height);
  println!("result = {}", result);
}
