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
  pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = ListNode::new(0);
    let mut pre = &mut dummy;
    let mut cur = head;
    while let Some(mut n1) = cur {
      cur = n1.next.take();
      if let Some(mut n2) = cur {
        cur = n2.next.take();
        n2.next = Some(n1);
        pre.next = Some(n2);
        pre = pre.next.as_mut().unwrap().next.as_mut().unwrap();
      } else {
        pre.next = Some(n1);
        pre = pre.next.as_mut().unwrap();
      }
    }
    dummy.next
  }
}

struct Solution;

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

fn main() {
  let mut n1 = ListNode::new(1);
  let mut n2 = ListNode::new(2);
  let mut n3 = ListNode::new(3);
  let n4 = ListNode::new(4);

  n3.next = Some(Box::new(n4));
  n2.next = Some(Box::new(n3));
  n1.next = Some(Box::new(n2));
  let head = Some(Box::new(n1));

  let result = Solution::swap_pairs(head);
  println!("result = {:?}", result);
}
