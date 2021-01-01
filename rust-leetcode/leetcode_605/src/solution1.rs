impl Solution {
  pub fn can_place_flowers(mut flowerbed: Vec<i32>, mut n: i32) -> bool {
    let length = flowerbed.len();
    for i in 0..length {
      if n == 0 {
        return true;
      }
      if flowerbed[i] == 1 {
        continue;
      }

      if i == 0 {
        if (i + 1 < length && flowerbed[i + 1] != 1) || (length == 1) {
          n -= 1;
          flowerbed[i] = 1;
        }
        continue;
      }
      if i == length - 1 {
        if flowerbed[i - 1] != 1 {
          n -= 1;
          flowerbed[i] = 1;
        }
        continue;
      }

      if flowerbed[i - 1] != 1 && flowerbed[i + 1] != 1 {
        n -= 1;
        flowerbed[i] = 1;
      }
    }
    if n <= 0 {
      true
    } else {
      false
    }
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1), true);
    assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2), false);
    assert_eq!(
      Solution::can_place_flowers(vec![1, 0, 0, 0, 1, 0, 0], 2),
      true
    );
    assert_eq!(Solution::can_place_flowers(vec![0], 1), true);
  }
}
