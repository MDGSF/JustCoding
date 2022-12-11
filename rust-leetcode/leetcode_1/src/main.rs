//use leetcode_1::solution1::Solution;
//use leetcode_1::solution2::Solution;
//use leetcode_1::solution3::Solution;
use leetcode_1::solution4::Solution;

fn main() {
    //let nums = vec![2, 7, 11, 15];
    //let target = 9;

    let nums = vec![3, 2, 4];
    let target = 6;
    let result = Solution::two_sum(nums, target);
    println!("result = {:?}", result);
}
