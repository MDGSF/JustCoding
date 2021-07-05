impl Solution {
  pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    let mut carry = 1;
    for i in (0..digits.len()).rev() {
      digits[i] += carry;
      carry = digits[i] / 10;
      digits[i] %= 10;
    }
    if carry > 0 {
      digits.insert(0, carry);
    }
    digits
  }
}

pub struct Solution;
