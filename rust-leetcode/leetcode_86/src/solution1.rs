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
  pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut small_head = Box::new(ListNode::new(0));
    let mut small_tail = small_head.as_mut();
    let mut large_head = Box::new(ListNode::new(0));
    let mut large_tail = large_head.as_mut();
    while let Some(mut node) = head {
      head = node.next.take();
      if node.val < x {
        small_tail.next = Some(node);
        small_tail = small_tail.next.as_mut().unwrap();
      } else {
        large_tail.next = Some(node);
        large_tail = large_tail.next.as_mut().unwrap();
      }
    }
    small_tail.next = large_head.next;
    small_head.next
  }
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNode { next: None, val }
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
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

    let mut onode1 = Box::new(ListNode::new(1));
    let mut onode2 = Box::new(ListNode::new(2));
    let mut onode3 = Box::new(ListNode::new(2));
    let mut onode4 = Box::new(ListNode::new(4));
    let mut onode5 = Box::new(ListNode::new(3));
    let onode6 = Box::new(ListNode::new(5));
    onode5.next = Some(onode6);
    onode4.next = Some(onode5);
    onode3.next = Some(onode4);
    onode2.next = Some(onode3);
    onode1.next = Some(onode2);

    assert_eq!(Solution::partition(Some(inode1), x), Some(onode1));
  }
}
