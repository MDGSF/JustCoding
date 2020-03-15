impl Solution {
  pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut tails = vec![0; nums.len()];
    let mut size = 0;
    for num in nums {
      let mut i = 0;
      let mut j = size;
      while i != j {
        let m = (i + j) / 2;
        if tails[m] < num {
          i = m + 1;
        } else {
          j = m;
        }
      }
      tails[i] = num;
      size = std::cmp::max(size, i + 1);
    }
    size as i32
  }
}

struct Solution;

fn main() {
  let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
  let result = Solution::length_of_lis(nums);
  println!("result = {}", result);
}
