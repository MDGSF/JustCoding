impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut arr = names
            .into_iter()
            .zip(heights.into_iter())
            .collect::<Vec<_>>();
        arr.sort_unstable_by_key(|(_, height)| -height);
        arr.into_iter().map(|(name, _)| name).collect()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let names = vec!["Mary".to_string(), "John".to_string(), "Emma".to_string()];
        let heights = vec![180, 165, 170];
        assert_eq!(
            Solution::sort_people(names, heights),
            vec!["Mary".to_string(), "Emma".to_string(), "John".to_string()]
        );
    }
}
