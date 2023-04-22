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
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = Some(Box::new(ListNode::new(0)));
        let mut p = result.as_mut().unwrap();
        let mut pre = 101;
        while let Some(mut node) = head {
            head = node.next.take();
            if (head.is_some() && head.as_ref().unwrap().val == node.val) || node.val == pre {
                pre = node.val;
            } else {
                pre = node.val;
                p.next = Some(node);
                p = p.next.as_mut().unwrap();
            }
        }
        result.as_mut().unwrap().next.take()
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
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
