impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let mut max_num = -1;
        for window in num.as_bytes().windows(3) {
            if window[0] == window[1] && window[1] == window[2] {
                let window_num = std::str::from_utf8(window).unwrap().parse::<i32>().unwrap();
                max_num = max_num.max(window_num);
            }
        }
        if max_num != -1 {
            format!("{:03}", max_num)
        } else {
            "".to_string()
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(
            Solution::largest_good_integer("6777133339".to_string()),
            "777".to_string()
        );
    }

    #[test]
    fn test02() {
        assert_eq!(
            Solution::largest_good_integer("2300019".to_string()),
            "000".to_string()
        );
    }

    #[test]
    fn test03() {
        assert_eq!(
            Solution::largest_good_integer("42352338".to_string()),
            "".to_string()
        );
    }
}
