use leetcode_2399::solution1::Solution;

fn main() {
    let s = "abaccb".to_string();
    let distance = vec![
        1, 3, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let result = Solution::check_distances(s, distance);
    println!("result: {}", result);
}
