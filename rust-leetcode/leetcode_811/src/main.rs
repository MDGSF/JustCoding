use leetcode_811::solution1::Solution;

fn main() {
  let cpdomains = vec!["9001 discuss.leetcode.com".to_string()];
  let result = Solution::subdomain_visits(cpdomains);
  println!("result = {:?}", result);
}
