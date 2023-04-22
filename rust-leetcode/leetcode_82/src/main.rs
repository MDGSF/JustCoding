use leetcode_82::solution1::{ListNode, Solution};

fn main() {
    let mut node1 = ListNode::new(1);
    let mut node2 = ListNode::new(2);
    let mut node3 = ListNode::new(3);
    let mut node4 = ListNode::new(3);
    let mut node5 = ListNode::new(4);
    let mut node6 = ListNode::new(4);
    let node7 = ListNode::new(5);

    node6.next = Some(Box::new(node7));
    node5.next = Some(Box::new(node6));
    node4.next = Some(Box::new(node5));
    node3.next = Some(Box::new(node4));
    node2.next = Some(Box::new(node3));
    node1.next = Some(Box::new(node2));

    let head = Some(Box::new(node1));

    let result = Solution::delete_duplicates(head);
    println!("result: {:?}", result);
}
