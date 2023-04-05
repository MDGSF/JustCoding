impl Solution {
    pub fn calculate_tax(brackets: Vec<Vec<i32>>, income: i32) -> f64 {
        let mut left = income;
        let mut tax = 0f64;
        for i in 0..brackets.len() {
            let upper = brackets[i][0];
            let percent = brackets[i][1];
            let calc_tax_money = if i == 0 {
                if income > upper {
                    upper
                } else {
                    income
                }
            } else {
                if income > upper {
                    upper - brackets[i - 1][0]
                } else {
                    income - brackets[i - 1][0]
                }
            };
            let cur_tax = calc_tax_money as f64 * (percent as f64 / 100f64);
            tax += cur_tax;
            left -= calc_tax_money;
            if left <= 0 {
                break;
            }
        }
        tax
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(
            Solution::calculate_tax(vec![vec![3, 50], vec![7, 10], vec![12, 25]], 10),
            2.65
        );
    }

    #[test]
    fn test02() {
        assert_eq!(
            Solution::calculate_tax(vec![vec![1, 0], vec![4, 25], vec![5, 50]], 2),
            0.25
        );
    }

    #[test]
    fn test03() {
        assert_eq!(Solution::calculate_tax(vec![vec![2, 50]], 0), 0f64);
    }
}
