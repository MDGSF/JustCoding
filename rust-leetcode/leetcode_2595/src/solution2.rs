impl Solution {
    pub fn even_odd_bit(mut n: i32) -> Vec<i32> {
        let mut idx = 0;
        let mut result = vec![0, 0];
        while n > 0 {
            if n & 1 == 1 {
                result[idx] += 1;
            }
            idx = 1 ^ idx;
            n = n >> 1;
        }
        result
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(Solution::even_odd_bit(17), vec![2, 0]);
        assert_eq!(Solution::even_odd_bit(2), vec![0, 1]);
    }
}
