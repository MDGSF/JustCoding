impl Solution {
    pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
        let is_vowel = |b: u8| -> bool {
            match b {
                b'a' | b'e' | b'i' | b'o' | b'u' => true,
                _ => false,
            }
        };

        let is_vowel_string = |s: &str| -> bool {
            let bs = s.as_bytes();
            is_vowel(bs[0]) && is_vowel(bs[bs.len() - 1])
        };

        (left..=right)
            .filter(|&idx| is_vowel_string(&words[idx as usize]))
            .count() as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let words = vec!["are".to_string(), "amy".to_string(), "u".to_string()];
        let left = 0;
        let right = 2;
        assert_eq!(Solution::vowel_strings(words, left, right), 2);
    }
}
