impl Solution {
    pub fn count_time(time: String) -> i32 {
        Solution::dfs(time, 0)
    }

    fn dfs(mut time: String, idx: usize) -> i32 {
        if idx == 5 {
            if Solution::is_valid(&time) {
                return 1;
            } else {
                return 0;
            }
        } else if idx == 2 {
            return Solution::dfs(time, idx + 1);
        } else {
            let bytes = unsafe { time.as_bytes_mut() };
            if bytes[idx] == b'?' {
                let mut count = 0;
                for i in 0..=9 {
                    bytes[idx] = b'0' + i;
                    count += Solution::dfs(String::from_utf8(bytes.to_vec()).unwrap(), idx + 1);
                }
                return count;
            } else {
                return Solution::dfs(time, idx + 1);
            }
        }
    }

    fn is_valid(time: &str) -> bool {
        let v = time.split(":").collect::<Vec<&str>>();
        let hour = v[0].parse::<i32>().unwrap();
        let minute = v[1].parse::<i32>().unwrap();
        hour >= 0 && hour < 24 && minute >= 0 && minute <= 59
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