impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut list = [0; 26];
        for c in sentence.chars() {
            list[c as usize - 97] += 1;
        }
        list.into_iter().all(|x| x != 0)
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
