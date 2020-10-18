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
    let mut length = 0;
    let mut cur = &head;

    while let Some(node) = cur {
      length += 1;
      cur = &node.next;
    }

    if length < n {
      return head;
    }

    let mut cur = &mut head;
    for i in 0..length - n {
      if let Some(node) = cur {
        cur = &mut node.next;
      }
    }

    if let Some(node) = cur {
      *cur = node.next.take();
    }

    head
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
