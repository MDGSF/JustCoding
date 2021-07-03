// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
  pub fn merge_two_lists(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
  ) -> Option<Box<ListNode>> {
    match (l1, l2) {
      (Some(l1), Some(l2)) => {
        let (mut small, big) = if l1.val < l2.val { (l1, l2) } else { (l2, l1) };
        let small_tail = small.next.take();
        small.next = Solution::merge_two_lists(small_tail, Some(big));
        return Some(small);
      }
      (None, Some(l2)) => {
        return Some(l2);
      }
      (Some(l1), None) => {
        return Some(l1);
      }
      _ => {
        return None;
      }
    }
  }
}

pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode { next: None, val }
  }
}
