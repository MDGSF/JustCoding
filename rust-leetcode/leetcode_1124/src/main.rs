use leetcode_1124::solution1::Solution;

fn main() {
    let hours = vec![9, 9, 6, 0, 6, 6, 9];
    let result = Solution::longest_wpi(hours);
    println!("result = {:?}", result);
}
