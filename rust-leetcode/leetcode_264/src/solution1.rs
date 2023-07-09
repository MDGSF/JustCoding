impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n];
        dp[0] = 1;
        let mut idx2 = 0;
        let mut idx3 = 0;
        let mut idx5 = 0;
        for i in 1..n {
            let n2 = dp[idx2] * 2;
            let n3 = dp[idx3] * 3;
            let n5 = dp[idx5] * 5;
            dp[i] = n2.min(n3).min(n5);
            if dp[i] == n2 {
                idx2 += 1;
            }
            if dp[i] == n3 {
                idx3 += 1;
            }
            if dp[i] == n5 {
                idx5 += 1;
            }
        }
        dp[n - 1]
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(Solution::nth_ugly_number(10), 12);
        assert_eq!(Solution::nth_ugly_number(1), 1);
    }
}
