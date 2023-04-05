use leetcode_2373::solution1::Solution;

fn main() {
    let grid = vec![
        vec![9, 9, 8, 1],
        vec![5, 6, 2, 6],
        vec![8, 2, 6, 4],
        vec![6, 2, 2, 2],
    ];
    let result = Solution::largest_local(grid);
    println!("result = {:?}", result);
}
