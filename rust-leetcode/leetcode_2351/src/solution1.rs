use std::collections::HashMap;

impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut map = HashMap::new();
        for c in s.chars() {
            if map.contains_key(&c) {
                return c;
            } else {
                map.insert(c, 1);
            }
        }
        unreachable!()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(Solution::repeated_character("abccbaacz".to_string()), 'c');
    }

    #[test]
    fn test02() {
        assert_eq!(Solution::repeated_character("abcdd".to_string()), 'd');
    }
}
