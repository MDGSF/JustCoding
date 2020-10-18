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
  pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = ListNode::new(-1);
    dummy.next = head;
    let mut dummy = Box::new(dummy);
    Self::travel(&mut dummy, n);
    dummy.next
  }

  fn travel(cur: &mut Box<ListNode>, n: i32) -> i32 {
    if let Some(child) = cur.next.as_mut() {
      let num = 1 + Self::travel(child, n);
      if num == n {
        cur.next = child.next.take();
      }
      return num;
    }
    0
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
  fn new(val: i32) -> Self {
    ListNode { next: None, val }
  }
}

struct Solution;

fn main() {
  println!("Hello, world!");
}
