impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut cur = Vec::new();
        let mut result = Vec::new();
        Self::recursion(&s, 3, &mut cur, &mut result);
        result
    }

    fn recursion(s: &str, point_num: usize, cur: &mut Vec<String>, result: &mut Vec<String>) {
        if point_num == 0 {
            if Self::is_valid_num(s) {
                cur.push(s.to_string());
                result.push(cur.join("."));
                cur.pop();
            }
            return;
        }

        let n = s.len();

        if n < point_num + 1 {
            // 数字太少
            return;
        }

        if n > (point_num + 1) * 3 {
            // 数字太多
            return;
        }

        for i in 1..=3 {
            if s.len() >= i {
                if Self::is_valid_num(&s[0..i]) {
                    cur.push(s[0..i].to_string());
                    Self::recursion(&s[i..n], point_num - 1, cur, result);
                    cur.pop();
                }
            }
        }
    }

    fn is_valid_num(s: &str) -> bool {
        if s.len() == 0 || s.len() > 3 {
            return false;
        }
        if s.len() > 1 && s.chars().nth(0) == Some('0') {
            return false;
        }
        if s.parse::<i32>().unwrap() > 255 {
            return false;
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
            Solution::restore_ip_addresses("0000".to_string()),
            vec!["0.0.0.0".to_string()]
        );
    }

    #[test]
    fn test02() {
        assert_eq!(
            Solution::restore_ip_addresses("25525511135".to_string()),
            vec!["255.255.11.135".to_string(), "255.255.111.35".to_string()]
        );
    }
}
