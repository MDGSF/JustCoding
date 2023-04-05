impl Solution {
    // 出现小写字母，就记录1
    // 出现大写字母，就记录2
    // 出现小写字母和大写字母，就记录3
    pub fn greatest_letter(s: String) -> String {
        let mut m = vec![0; 26];
        for b in s.bytes() {
            if b >= b'a' && b <= b'z' {
                let idx = (b - b'a') as usize;
                if m[idx] == 0 {
                    m[idx] = 1;
                } else if m[idx] == 2 {
                    m[idx] = 3;
                }
            } else {
                let idx = (b - b'A') as usize;
                if m[idx] == 0 {
                    m[idx] = 2;
                } else if m[idx] == 1 {
                    m[idx] = 3;
                }
            };
        }

        for i in (0..26).rev() {
            if m[i] == 3 {
                return ((b'A' + i as u8) as char).to_string();
            }
        }

        "".to_string()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(
            Solution::greatest_letter("lEeTcOdE".to_string()),
            "E".to_string()
        );
    }

    #[test]
    fn test02() {
        assert_eq!(
            Solution::greatest_letter("arRAzFif".to_string()),
            "R".to_string()
        );
    }

    #[test]
    fn test03() {
        assert_eq!(
            Solution::greatest_letter("AbCdEfGhIjK".to_string()),
            "".to_string()
        );
    }
}
