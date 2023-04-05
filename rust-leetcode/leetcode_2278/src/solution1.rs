impl Solution {
    pub fn percentage_letter(s: String, letter: char) -> i32 {
        let mut count = 0;
        for c in s.chars() {
            if c == letter {
                count += 1;
            }
        }
        ((count as f64 / s.len() as f64) * 100f64) as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(Solution::percentage_letter("foobar".to_string(), 'o'), 33);
        assert_eq!(Solution::percentage_letter("jjjj".to_string(), 'k'), 0);
    }
}
