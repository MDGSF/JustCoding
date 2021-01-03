use leetcode_86::solution1::*;

fn main() {
  let x = 3;

  let mut inode1 = Box::new(ListNode::new(1));
  let mut inode2 = Box::new(ListNode::new(4));
  let mut inode3 = Box::new(ListNode::new(3));
  let mut inode4 = Box::new(ListNode::new(2));
  let mut inode5 = Box::new(ListNode::new(5));
  let inode6 = Box::new(ListNode::new(2));
  inode5.next = Some(inode6);
  inode4.next = Some(inode5);
  inode3.next = Some(inode4);
  inode2.next = Some(inode3);
  inode1.next = Some(inode2);

  let result = Solution::partition(Some(inode1), x);

  println!("result = {:?}", result);
}
