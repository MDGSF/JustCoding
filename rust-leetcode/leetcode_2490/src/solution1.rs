impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let words: Vec<&str> = sentence.split(' ').collect();
        let first_word = words[0];
        let last_word = words[words.len() - 1];
        let first_char = first_word.chars().next();
        let last_char = last_word.chars().last();
        if first_char != last_char {
            return false;
        }

        for window in words.windows(2) {
            let pre_char = window[0].chars().last();
            let next_char = window[1].chars().next();
            if pre_char != next_char {
                return false;
            }
        }

        true
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(
            Solution::is_circular_sentence("leetcode exercises sound delightful".to_string()),
            true
        );
    }

    #[test]
    fn test02() {
        assert_eq!(Solution::is_circular_sentence("eetcode".to_string()), true);
    }

    #[test]
    fn test03() {
        assert_eq!(
            Solution::is_circular_sentence("Leetcode is cool".to_string()),
            false
        );
    }
}
