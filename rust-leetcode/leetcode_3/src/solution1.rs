use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_len = 0;
        let mut cur_len = 0;
        let mut left = 0;
        let mut windows = HashSet::new();
        for (_, c) in s.chars().enumerate() {
            while windows.contains(&c) {
                windows.remove(&s.chars().nth(left).unwrap());
                left += 1;
                cur_len -= 1;
            }

            cur_len += 1;
            windows.insert(c);

            if cur_len > max_len {
                max_len = cur_len;
            }
        }
        max_len
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );

        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );

        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }
}
