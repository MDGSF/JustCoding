use std::cmp::max;
use std::cmp::min;
use std::str::FromStr;

impl Solution {
    pub fn count_days_together(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        const DAY_OF_MONTH: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut prefix_sum = vec![0];
        for day in DAY_OF_MONTH.iter() {
            prefix_sum.push(prefix_sum[prefix_sum.len() - 1] + day);
        }

        let to_days = |date: &str| -> i32 {
            prefix_sum[(i32::from_str(&date[0..2]).unwrap() - 1) as usize]
                + i32::from_str(&date[3..5]).unwrap()
        };

        let arrive_alice = to_days(&arrive_alice);
        let leave_alice = to_days(&leave_alice);
        let arrive_bob = to_days(&arrive_bob);
        let leave_bob = to_days(&leave_bob);

        max(
            0,
            min(leave_alice, leave_bob) - max(arrive_alice, arrive_bob) + 1,
        )
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let arrive_alice = "08-15".to_string();
        let leave_alice = "08-18".to_string();
        let arrive_bob = "08-16".to_string();
        let leave_bob = "08-19".to_string();

        assert_eq!(
            Solution::count_days_together(arrive_alice, leave_alice, arrive_bob, leave_bob),
            3
        );
    }

    #[test]
    fn test02() {
        let arrive_alice = "10-01".to_string();
        let leave_alice = "10-31".to_string();
        let arrive_bob = "11-01".to_string();
        let leave_bob = "12-31".to_string();

        assert_eq!(
            Solution::count_days_together(arrive_alice, leave_alice, arrive_bob, leave_bob),
            0
        );
    }
}
