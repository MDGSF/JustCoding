/*
 * @lc app=leetcode.cn id=2 lang=javascript
 *
 * [2] 两数相加
 *
 * https://leetcode-cn.com/problems/add-two-numbers/description/
 *
 * algorithms
 * Medium (35.79%)
 * Likes:    3231
 * Dislikes: 0
 * Total Accepted:    233.6K
 * Total Submissions: 651.9K
 * Testcase Example:  '[2,4,3]\n[5,6,4]'
 *
 * 给出两个 非空 的链表用来表示两个非负的整数。其中，它们各自的位数是按照 逆序 的方式存储的，并且它们的每个节点只能存储 一位 数字。
 *
 * 如果，我们将这两个数相加起来，则会返回一个新的链表来表示它们的和。
 *
 * 您可以假设除了数字 0 之外，这两个数都不会以 0 开头。
 *
 * 示例：
 *
 * 输入：(2 -> 4 -> 3) + (5 -> 6 -> 4)
 * 输出：7 -> 0 -> 8
 * 原因：342 + 465 = 807
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
var addTwoNumbers = function(l1, l2) {
  let p1 = l1;
  let p2 = l2;
  let p3Head = null;
  let p3Tail = null;
  let carry = 0;
  while (p1 !== null || p2 !== null || carry !== 0) {
    let newValue = carry;
    if (p1 !== null) {
      newValue += p1.val;
    }
    if (p2 !== null) {
      newValue += p2.val;
    }
    carry = Math.floor(newValue / 10);
    newValue = newValue % 10;
    const newNode = new ListNode(newValue);

    if (p3Head === null) {
      p3Head = newNode;
    } else {
      p3Tail.next = newNode;
    }
    p3Tail = newNode;

    p1 = p1 === null ? null : p1.next;
    p2 = p2 === null ? null : p2.next;
  }
  return p3Head;
};
// @lc code=end
