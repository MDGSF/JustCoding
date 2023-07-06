/// Compute power of two greater than or equal to 'n'
pub fn find_next_power_of_2(mut n: u32) -> u32 {
    // decrement 'n' (to handle the case when 'n' itself is power of 2)
    n = n - 1;

    // set all bits on the right-hand side of the most significant set bit to 1
    n |= n >> 1;
    n |= n >> 2;
    n |= n >> 4;
    n |= n >> 8;
    n |= n >> 16;

    n + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(find_next_power_of_2(2), 2);
        assert_eq!(find_next_power_of_2(3), 4);
        assert_eq!(find_next_power_of_2(4), 4);
        assert_eq!(find_next_power_of_2(5), 8);
        assert_eq!(find_next_power_of_2(6), 8);
        assert_eq!(find_next_power_of_2(7), 8);
        assert_eq!(find_next_power_of_2(8), 8);
    }

    #[test]
    fn test02() {
        assert_eq!(find_next_power_of_2(9), 16);
        assert_eq!(find_next_power_of_2(10), 16);
        assert_eq!(find_next_power_of_2(11), 16);
        assert_eq!(find_next_power_of_2(12), 16);
        assert_eq!(find_next_power_of_2(13), 16);
        assert_eq!(find_next_power_of_2(14), 16);
        assert_eq!(find_next_power_of_2(15), 16);
        assert_eq!(find_next_power_of_2(16), 16);
    }

    #[test]
    fn test03() {
        for i in 17..=32 {
            assert_eq!(find_next_power_of_2(i), 32);
        }
    }
}
