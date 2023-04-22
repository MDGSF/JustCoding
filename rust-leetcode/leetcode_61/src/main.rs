use leetcode_61::solution1::{Solution, ListNode};

fn main() {
    let mut node1 = ListNode::new(1);
    let mut node2 = ListNode::new(2);
    let mut node3 = ListNode::new(3);
    let mut node4 = ListNode::new(4);
    let node5 = ListNode::new(5);

    node4.next = Some(Box::new(node5));
    node3.next = Some(Box::new(node4));
    node2.next = Some(Box::new(node3));
    node1.next = Some(Box::new(node2));
    let head = Some(Box::new(node1));

    let result = Solution::rotate_right(head, 2);
    println!("{:?}", result);
}
