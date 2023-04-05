use leetcode_2325::solution1::Solution;

fn main() {
    let result = Solution::decode_message(
        "the quick brown fox jumps over the lazy dog".to_string(),
        "vkbs bs t suepuv".to_string(),
    );
    println!("result: {}", result);
}
