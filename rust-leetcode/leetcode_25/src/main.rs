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
  pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut head = head;
    let (mut tail, ok) = Solution::forward_k_steps(&mut head, k);
    if !ok {
      head
    } else {
      head = Solution::reverse_list(head, None);
      tail = Solution::reverse_k_group(tail, k);
      Solution::connect(head, tail)
    }
  }

  fn forward_k_steps(src: &mut Option<Box<ListNode>>, k: i32) -> (Option<Box<ListNode>>, bool) {
    if let Some(src) = src {
      if k == 1 {
        (src.next.take(), true)
      } else {
        Solution::forward_k_steps(&mut src.next, k - 1)
      }
    } else {
      (None, false)
    }
  }

  fn reverse_list(
    src: Option<Box<ListNode>>,
    built: Option<Box<ListNode>>,
  ) -> Option<Box<ListNode>> {
    if let Some(mut src1) = src {
      let tail = src1.next.take();
      std::mem::replace(&mut src1.next, built);
      Solution::reverse_list(tail, Some(src1))
    } else {
      built
    }
  }

  fn connect(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if let Some(mut head) = list1 {
      let tail = head.next.take();
      std::mem::replace(&mut head.next, Solution::connect(tail, list2));
      Some(head)
    } else {
      list2
    }
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
