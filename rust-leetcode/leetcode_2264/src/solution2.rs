impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        num.as_bytes()
            .windows(3)
            .fold(Some(i32::MIN), |max_num, window| {
                if window[0] == window[1] && window[1] == window[2] {
                    Some(
                        max_num
                            .unwrap()
                            .max(std::str::from_utf8(window).unwrap().parse::<i32>().unwrap()),
                    )
                } else {
                    max_num
                }
            })
            .map(|num| {
                if num == i32::MIN {
                    "".to_string()
                } else {
                    format!("{:03}", num)
                }
            })
            .unwrap()
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
