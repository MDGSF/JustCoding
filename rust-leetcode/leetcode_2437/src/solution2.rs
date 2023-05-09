impl Solution {
    pub fn count_time(time: String) -> i32 {
        let t = time.chars().collect::<Vec<_>>();

        let hour = match (t[0], t[1]) {
            ('?', '?') => 24,
            ('?', '0'..='3') => 3,
            ('?', '4'..='9') => 2,
            ('0' | '1', '?') => 10,
            ('2', '?') => 4,
            _ => 1,
        };

        let minute = match (t[3], t[4]) {
            ('?', '?') => 60,
            ('?', _) => 6,
            (_, '?') => 10,
            _ => 1,
        };

        hour * minute
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(Solution::count_time("?5:00".to_string()), 2);
    }

    #[test]
    fn test02() {
        assert_eq!(Solution::count_time("0?:0?".to_string()), 100);
    }

    #[test]
    fn test03() {
        assert_eq!(Solution::count_time("??:??".to_string()), 1440);
    }
}