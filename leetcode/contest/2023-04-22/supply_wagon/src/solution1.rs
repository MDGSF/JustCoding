impl Solution {
    pub fn supply_wagon(mut supplies: Vec<i32>) -> Vec<i32> {
        let n = supplies.len();
        let expected_n = n / 2;
        loop {
            if supplies.len() <= expected_n {
                break;
            }

            let mut min_num = supplies[0] + supplies[1];
            let mut min_idx = 0;
            for (i, window) in supplies.windows(2).enumerate() {
                let num = window[0] + window[1];
                if num < min_num {
                    min_num = num;
                    min_idx = i;
                }
            }

            supplies[min_idx] += supplies[min_idx + 1];
            supplies.remove(min_idx + 1);
        }
        supplies
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(Solution::supply_wagon(vec![7, 3, 6, 1, 8]), vec![10, 15]);
    }

    #[test]
    fn test02() {
        assert_eq!(Solution::supply_wagon(vec![1, 3, 1, 5]), vec![5, 5]);
    }


    #[test]
    fn test03() {
        assert_eq!(Solution::supply_wagon(vec![1, 3]), vec![4]);
    }

    #[test]
    fn test04() {
        assert_eq!(Solution::supply_wagon(vec![1, 2, 3]), vec![6]);
    }
}
