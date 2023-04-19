use std::collections::HashMap;

impl Solution {
    pub fn similar_pairs(words: Vec<String>) -> i32 {
        let mut map = HashMap::with_capacity(words.len());
        let mut count = 0;
        words.iter().for_each(|word| {
            let key = word.bytes().fold(0, |acc, c| acc | 1 << (c - b'a'));
            let cnt = map.entry(key).or_insert(0);
            count += *cnt;
            *cnt += 1;
        });
        count
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(
            Solution::similar_pairs(vec![
                "aba".to_string(),
                "aabb".to_string(),
                "abcd".to_string(),
                "bac".to_string(),
                "aabc".to_string()
            ]),
            2
        );
    }
}
