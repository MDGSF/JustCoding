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
  pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut new_head = None;
    while let Some(mut node) = head {
      head = node.next.take();
      node.next = new_head;
      new_head = Some(node);
    }
    new_head
  }

  pub fn reverse_list_1(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut pre = None;
    let mut cur = head;
    while let Some(mut node) = cur {
      let next = node.next;
      node.next = pre;
      pre = Some(node);
      cur = next;
    }
    pre
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
  let mut n1 = ListNode::new(1);
  let mut n2 = ListNode::new(2);
  let mut n3 = ListNode::new(3);
  let mut n4 = ListNode::new(4);
  let n5 = ListNode::new(5);

  n4.next = Some(Box::new(n5));
  n3.next = Some(Box::new(n4));
  n2.next = Some(Box::new(n3));
  n1.next = Some(Box::new(n2));
  let head = Some(Box::new(n1));

  let result = Solution::reverse_list(head);
  println!("result = {:?}", result);
}
