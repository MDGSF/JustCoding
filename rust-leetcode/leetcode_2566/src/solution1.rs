impl Solution {
    pub fn min_max_difference(num: i32) -> i32 {
        let num_str = num.to_string();
        let max_num = match num_str.chars().find(|&x| x != '9') {
            Some(c) => num_str.replace(&c.to_string(), "9").parse::<i32>().unwrap(),
            None => num,
        };
        let min_num = match num_str.chars().find(|&x| x != '0') {
            Some(c) => num_str.replace(&c.to_string(), "0").parse::<i32>().unwrap(),
            None => num,
        };
        max_num - min_num
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(Solution::min_max_difference(11891), 99009);
    }

    #[test]
    fn test02() {
        assert_eq!(Solution::min_max_difference(90), 99);
    }
}
