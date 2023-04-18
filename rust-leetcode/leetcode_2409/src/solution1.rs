#[derive(Eq, PartialEq)]
pub struct MonDay {
    month: i32,
    day: i32,
}

impl From<String> for MonDay {
    fn from(month_day: String) -> Self {
        Self::from(month_day.as_str())
    }
}

impl From<&str> for MonDay {
    fn from(month_day: &str) -> Self {
        let (month, day) = month_day.split_once('-').unwrap();
        MonDay {
            month: month.parse::<i32>().unwrap(),
            day: day.parse::<i32>().unwrap(),
        }
    }
}

impl std::cmp::Ord for MonDay {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.month == other.month {
            return self.day.cmp(&other.day);
        } else if self.month < other.month {
            return std::cmp::Ordering::Less;
        } else {
            // self.month > other.month
            return std::cmp::Ordering::Greater;
        }
    }
}

impl std::cmp::PartialOrd for MonDay {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn month_day_distance(start: MonDay, end: MonDay) -> i32 {
    if start.month == end.month {
        return end.day - start.day + 1;
    }

    let mut distance = 0;
    let day_of_month = vec![0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    for cur_month in start.month..=end.month {
        if cur_month == start.month {
            distance += day_of_month[start.month as usize] - start.day + 1;
        } else if cur_month == end.month {
            distance += end.day;
        } else {
            distance += day_of_month[cur_month as usize];
        }
    }
    distance
}

impl Solution {
    pub fn count_days_together(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        let arrive_alice = MonDay::from(arrive_alice);
        let leave_alice = MonDay::from(leave_alice);
        let arrive_bob = MonDay::from(arrive_bob);
        let leave_bob = MonDay::from(leave_bob);

        // 没有交集
        if leave_alice < arrive_bob || leave_bob < arrive_alice {
            return 0;
        }

        // alice 被包含在 bob 里面
        if arrive_alice >= arrive_bob && leave_alice <= leave_bob {
            return month_day_distance(arrive_alice, leave_alice);
        }

        // bob 被包含在 alice 里面
        if arrive_bob >= arrive_alice && leave_bob <= leave_alice {
            return month_day_distance(arrive_bob, leave_bob);
        }

        // 部分交集，alice 在前面
        if arrive_alice < arrive_bob && arrive_bob <= leave_alice && leave_alice < leave_bob {
            return month_day_distance(arrive_bob, leave_alice);
        }

        // 部分交集，bob 在前面
        if arrive_bob < arrive_alice && arrive_alice <= leave_bob && leave_bob < leave_alice {
            return month_day_distance(arrive_alice, leave_bob);
        }

        0
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
