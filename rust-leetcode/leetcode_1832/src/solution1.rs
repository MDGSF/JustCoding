use std::collections::HashMap;

impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut m = HashMap::new();
        let mut count = 0;

        for c in sentence.chars() {
            if !m.contains_key(&c) {
                m.insert(c, 1);
                count += 1;
            }
        }

        count == 26
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::check_if_pangram("thequickbrownfoxjumpsoverthelazydog".to_string()),
            true
        );

        assert_eq!(Solution::check_if_pangram("leetcode".to_string()), false);
    }
}
