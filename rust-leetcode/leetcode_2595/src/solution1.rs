impl Solution {
    pub fn even_odd_bit(mut n: i32) -> Vec<i32> {
        let mut even = 0;
        let mut odd = 0;
        let mut i = 0;
        while n > 0 {
            let bit = n & 0x01;
            if bit == 1 {
                if i % 2 == 0 {
                    even += 1;
                } else {
                    odd += 1;
                }
            }
            n = n >> 1;
            i += 1;
        }
        vec![even, odd]
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
