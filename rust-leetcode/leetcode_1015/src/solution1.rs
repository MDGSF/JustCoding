use std::collections::HashSet;

impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        let mut resid = 1 % k;
        let mut len = 1;
        let mut set = HashSet::new();
        set.insert(resid);
        while resid != 0 {
            resid = (resid * 10 + 1) % k;
            len += 1;
            if set.contains(&resid) {
                return -1;
            }
            set.insert(resid);
        }
        len
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(Solution::smallest_repunit_div_by_k(1), 1);
    }

    #[test]
    fn test02() {
        assert_eq!(Solution::smallest_repunit_div_by_k(2), -1);
    }

    #[test]
    fn test03() {
        assert_eq!(Solution::smallest_repunit_div_by_k(3), 3);
    }
}
