impl Solution {
    pub fn digit_count(num: String) -> bool {
        let mut cnt: Vec<u8> = vec![0; 10];
        num.bytes().for_each(|c| cnt[(c - b'0') as usize] += 1);
        num.char_indices()
            .all(|(idx, c)| (c as u8 - b'0') == cnt[idx])
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(Solution::digit_count("1210".to_string()), true);
        assert_eq!(Solution::digit_count("030".to_string()), false);
    }
}
