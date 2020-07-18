impl Solution {
  fn has_cycle(head: Option<Box<ListNode>>) -> bool {
    let mut slow = &head;
    let mut fast = &head;
    while slow.is_some() && fast.is_some() && fast.as_ref().unwrap().next.is_some() {
      slow = &(slow.as_ref().unwrap().next);
      fast = &(fast.as_ref().unwrap().next);
      fast = &(fast.as_ref().unwrap().next);
      if slow == fast {
        return true;
      }
    }

    false
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
  println!("Hello, world!");
}
