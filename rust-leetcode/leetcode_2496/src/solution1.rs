impl Solution {
    pub fn maximum_value(strs: Vec<String>) -> i32 {
        strs.iter()
            .map(|s| {
                if s.chars().any(|c| c.is_lowercase()) {
                    s.len() as i32
                } else {
                    s.parse::<i32>().unwrap()
                }
            })
            .max()
            .unwrap()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(
            Solution::maximum_value(vec![
                "alic3".to_string(),
                "bob".to_string(),
                "3".to_string(),
                "4".to_string(),
                "00000".to_string()
            ]),
            5
        );
    }
}
