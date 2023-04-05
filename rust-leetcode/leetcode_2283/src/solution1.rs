impl Solution {
    pub fn digit_count(num: String) -> bool {
        let n = num.len();
        let mut m1 = vec![0; n];
        let mut m2 = vec![0; 10];
        for (idx, c) in num.chars().enumerate() {
            let number = c.to_digit(10).unwrap();
            m1[idx] = number;
            m2[number as usize] += 1;
        }
        for i in 0..n {
            if m1[i] != m2[i] {
                return false;
            }
        }
        true
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(Solution::digit_count("1210".to_string()), true);
        assert_eq!(Solution::digit_count("030".to_string()), false);
    }
}
