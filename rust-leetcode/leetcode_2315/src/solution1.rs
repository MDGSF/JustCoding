impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        let mut count = 0;
        let mut need_count = true;
        for c in s.chars() {
            if c == '|' {
                need_count = !need_count;
            } else if c == '*' {
                if need_count {
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
            Solution::count_asterisks("l|*e*et|c**o|*de|".to_string()),
            2
        );
        assert_eq!(Solution::count_asterisks("iamprogrammer".to_string()), 0);
        assert_eq!(
            Solution::count_asterisks("yo|uar|e**|b|e***au|tifu|l".to_string()),
            5
        );
    }
}
