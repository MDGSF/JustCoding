use leetcode_93::solution1::Solution;

fn main() {
    let s = "101023".to_string();
    let result = Solution::restore_ip_addresses(s);
    println!("result: {:?}", result);
}
