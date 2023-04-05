use std::collections::HashMap;

impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        let mut key_map: HashMap<char, char> = HashMap::new();
        let mut start = b'a';
        for c in key.chars() {
            if c == ' ' {
                continue;
            }

            if !key_map.contains_key(&c) {
                key_map.insert(c, start as char);
                start += 1;
                if start > b'z' {
                    break;
                }
            }
        }

        let mut result = String::new();
        for c in message.chars() {
            if c == ' ' {
                result.push(' ');
            } else {
                result.push(key_map[&c]);
            }
        }
        result
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(
            Solution::decode_message(
                "the quick brown fox jumps over the lazy dog".to_string(),
                "vkbs bs t suepuv".to_string()
            ),
            "this is a secret".to_string()
        );
    }

    #[test]
    fn test02() {
        assert_eq!(
            Solution::decode_message(
                "eljuxhpwnyrdgtqkviszcfmabo".to_string(),
                "zwx hnfx lqantp mnoeius ycgk vcnjrdb".to_string()
            ),
            "the five boxing wizards jump quickly".to_string()
        );
    }
}
