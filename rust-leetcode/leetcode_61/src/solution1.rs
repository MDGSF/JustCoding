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
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let n: usize = Self::list_len(&head);
        let k: usize = k as usize % n;
        let (first, second) = Self::list_split(head, n - k);
        Self::list_concat(second, first)
    }

    fn list_len(node: &Option<Box<ListNode>>) -> usize {
        match node {
            Some(node) => 1 + Self::list_len(&node.next),
            None => 0,
        }
    }

    fn list_split(
        head: Option<Box<ListNode>>,
        first_len: usize,
    ) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        if first_len == 0 {
            (None, head)
        } else {
            if let Some(mut head) = head {
                let tail = head.next.take();
                let (first, second) = Self::list_split(tail, first_len - 1);
                let _ = std::mem::replace(&mut head.next, first);
                (Some(head), second)
            } else {
                (None, None)
            }
        }
    }

    fn list_concat(
        first: Option<Box<ListNode>>,
        second: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if let Some(mut head) = first {
            let tail = head.next.take();
            let _ = std::mem::replace(&mut head.next, Self::list_concat(tail, second));
            Some(head)
        } else {
            second
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
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
