use std::collections::HashMap;

impl Solution {
    pub fn longest_wpi(hours: Vec<i32>) -> i32 {
        let mut m: HashMap<i32, usize> = HashMap::new();
        let mut max_length = 0;
        let mut presum = 0;
        let n = hours.len();
        for i in 0..n {
            presum += if hours[i] > 8 { 1 } else { -1 };
            if presum > 0 {
                max_length = i + 1;
            } else {
                if !m.contains_key(&presum) {
                    m.insert(presum, i);
                }
                if m.contains_key(&(presum - 1)) {
                    max_length = max_length.max(i - m[&(presum - 1)]);
                }
            }
        }
        max_length as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let hours = vec![9, 9, 6, 0, 6, 6, 9];
        let result = Solution::longest_wpi(hours);
        assert_eq!(result, 3);
    }

    #[test]
    fn test02() {
        let hours = vec![6, 6, 6];
        let result = Solution::longest_wpi(hours);
        assert_eq!(result, 0);
    }
}
