use leetcode_sw_07::s2::*;

fn main() {
  println!("Hello, world!");

  let preorder = vec![3,9,20,15,7];
  let inorder = vec![9,3,15,20,7];
  let result = Solution::build_tree(preorder, inorder);
  println!("{:?}", result);
}
