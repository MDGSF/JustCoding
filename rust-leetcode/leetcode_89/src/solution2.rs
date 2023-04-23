impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        (0..1 << n).map(|i| i ^ (i >> 1)).collect()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(Solution::gray_code(2), vec![0, 1, 3, 2]);
    }
}
