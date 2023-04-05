impl Solution {
    pub fn fill_cups(mut amount: Vec<i32>) -> i32 {
        amount.sort();
        if amount[0] + amount[1] <= amount[2] {
            amount[2]
        } else {
            (amount[0] + amount[1] + amount[2] + 1) / 2
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(Solution::fill_cups(vec![1, 4, 2]), 4);
    }

    #[test]
    fn test02() {
        assert_eq!(Solution::fill_cups(vec![5, 4, 4]), 7);
    }

    #[test]
    fn test03() {
        assert_eq!(Solution::fill_cups(vec![5, 0, 0]), 5);
    }
}
