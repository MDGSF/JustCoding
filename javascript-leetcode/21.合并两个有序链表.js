/*
 * @lc app=leetcode.cn id=21 lang=javascript
 *
 * [21] 合并两个有序链表
 *
 * https://leetcode-cn.com/problems/merge-two-sorted-lists/description/
 *
 * algorithms
 * Easy (57.62%)
 * Likes:    679
 * Dislikes: 0
 * Total Accepted:    127.8K
 * Total Submissions: 220.8K
 * Testcase Example:  '[1,2,4]\n[1,3,4]'
 *
 * 将两个有序链表合并为一个新的有序链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。
 *
 * 示例：
 *
 * 输入：1->2->4, 1->3->4
 * 输出：1->1->2->3->4->4
 *
 *
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 * function ListNode(val) {
 *     this.val = val;
 *     this.next = null;
 * }
 */
/**
 * @param {ListNode} l1
 * @param {ListNode} l2
 * @return {ListNode}
 */
var mergeTwoLists = function(l1, l2) {
  let dummy = new ListNode();
  let cur = dummy;
  let p1 = l1;
  let p2 = l2;
  while ((p1 !== null) & (p2 !== null)) {
    if (p1.val < p2.val) {
      cur.next = p1;
      p1 = p1.next;
    } else {
      cur.next = p2;
      p2 = p2.next;
    }
    cur = cur.next;
  }

  if (p1 !== null) {
    cur.next = p1;
  }

  if (p2 !== null) {
    cur.next = p2;
  }

  return dummy.next;
};
// @lc code=end
