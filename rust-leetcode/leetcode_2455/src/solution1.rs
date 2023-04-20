impl Solution {
    pub fn average_value(nums: Vec<i32>) -> i32 {
        nums.iter()
            .filter(|&&num| num % 2 == 0 && num % 3 == 0)
            .fold(Some((0, 0)), |acc, num| {
                Some((acc.unwrap().0 + num, acc.unwrap().1 + 1))
            })
            .map(|(sum, count)| if count == 0 { 0 } else { sum / count })
            .unwrap()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(Solution::average_value(vec![1, 3, 6, 10, 12, 15]), 9);
    }

    #[test]
    fn test02() {
        assert_eq!(Solution::average_value(vec![1, 2, 4, 7, 10]), 0);
    }
}
