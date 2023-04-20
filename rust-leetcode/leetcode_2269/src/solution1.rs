impl Solution {
    pub fn divisor_substrings(num: i32, k: i32) -> i32 {
        num.to_string()
            .as_bytes()
            .windows(k as usize)
            .map(|window| std::str::from_utf8(window).unwrap().parse::<i32>().unwrap())
            .filter(|&n| n != 0 && num % n == 0)
            .count() as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(Solution::divisor_substrings(240, 2), 2);
    }
}
