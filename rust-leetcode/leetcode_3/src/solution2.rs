impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s_len = s.len();
        if s_len == 0 {
            return 0;
        }
        let s = s.as_bytes();
        let mut max_len = 0;
        let mut left = 0;
        let mut map = std::collections::HashMap::<u8, usize>::new();
        for i in 0..s_len {
            if let Some(&index) = map.get(&s[i]) {
                left = std::cmp::max(left, index + 1);
            }
            if i - left + 1 > max_len {
                max_len = i - left + 1;
            }
            map.insert(s[i], i);
        }
        max_len as i32
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
