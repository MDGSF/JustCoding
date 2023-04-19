use std::collections::HashSet;

impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut set = HashSet::new();
        for c in s.chars() {
            if set.contains(&c) {
                return c;
            } else {
                set.insert(c);
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
