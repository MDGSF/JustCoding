use leetcode_605::solution1::Solution;

fn main() {
  //let flowerbed = vec![1, 0, 0, 0, 1];
  //let n = 1;
  let flowerbed = vec![1, 0, 0, 0, 1, 0, 0];
  let n = 2;
  let result = Solution::can_place_flowers(flowerbed, n);
  println!("result = {}", result);
}
