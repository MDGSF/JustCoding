impl Solution {
    pub fn min_number_of_hours(
        initial_energy: i32,
        initial_experience: i32,
        energy: Vec<i32>,
        experience: Vec<i32>,
    ) -> i32 {
        let energy_sum = energy.iter().sum::<i32>();
        let energy_result = if energy_sum + 1 > initial_energy {
            energy_sum + 1 - initial_energy
        } else {
            0
        };

        let mut experience_result = 0;
        let mut experience_sum = initial_experience;
        for x in experience.iter() {
            if experience_sum > x + 1 {
                experience_sum += x;
            } else {
                experience_result += x + 1 - experience_sum;
                experience_sum = x + 1 + x;
            }
        }

        energy_result + experience_result
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let initial_energy = 5;
        let initial_experience = 3;
        let energy = vec![1, 4, 3, 2];
        let experience = vec![2, 6, 3, 1];
        assert_eq!(
            Solution::min_number_of_hours(initial_energy, initial_experience, energy, experience),
            8
        );
    }

    #[test]
    fn test02() {
        let initial_energy = 2;
        let initial_experience = 4;
        let energy = vec![1];
        let experience = vec![3];
        assert_eq!(
            Solution::min_number_of_hours(initial_energy, initial_experience, energy, experience),
            0
        );
    }
}
