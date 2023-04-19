use std::collections::BTreeSet;

impl Solution {
    pub fn similar_pairs(words: Vec<String>) -> i32 {
        let hash = |s: &str| -> String {
            s.chars()
                .collect::<BTreeSet<char>>()
                .iter()
                .collect::<String>()
        };
        let keys = words
            .iter()
            .map(|word| hash(&word))
            .collect::<Vec<String>>();
        let mut count = 0;
        for i in 0..keys.len() {
            for j in i + 1..keys.len() {
                if keys[i] == keys[j] {
                    count += 1;
                }
            }
        }
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
