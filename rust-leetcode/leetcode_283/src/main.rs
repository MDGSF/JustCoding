impl Solution {
  pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut j = 0;
    for i in 0..nums.len() {
      if nums[i] != 0 {
        if j != i {
          nums.swap(i, j);
        }
        j += 1;
      }
    }
  }
}

struct Solution {}

fn main() {
  let mut nums = vec![0, 1, 0, 3, 12];
  Solution::move_zeroes(&mut nums);
  println!("{:?}", nums);
}
