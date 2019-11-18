/*
 * @lc app=leetcode.cn id=160 lang=javascript
 *
 * [160] 相交链表
 *
 * https://leetcode-cn.com/problems/intersection-of-two-linked-lists/description/
 *
 * algorithms
 * Easy (36.67%)
 * Likes:    435
 * Dislikes: 0
 * Total Accepted:    51K
 * Total Submissions: 103.3K
 * Testcase Example:  '8\n[4,1,8,4,5]\n[5,0,1,8,4,5]\n2\n3'
 *
 * 编写一个程序，找到两个单链表相交的起始节点。
 *
 * 如下面的两个链表：
 *
 *
 *
 * 在节点 c1 开始相交。
 *
 *
 *
 * 示例 1：
 *
 *
 *
 * 输入：intersectVal = 8, listA = [4,1,8,4,5], listB = [5,0,1,8,4,5], skipA = 2,
 * skipB = 3
 * 输出：Reference of the node with value = 8
 * 输入解释：相交节点的值为 8 （注意，如果两个列表相交则不能为 0）。从各自的表头开始算起，链表 A 为 [4,1,8,4,5]，链表 B 为
 * [5,0,1,8,4,5]。在 A 中，相交节点前有 2 个节点；在 B 中，相交节点前有 3 个节点。
 *
 *
 *
 *
 * 示例 2：
 *
 *
 *
 * 输入：intersectVal = 2, listA = [0,9,1,2,4], listB = [3,2,4], skipA = 3, skipB
 * = 1
 * 输出：Reference of the node with value = 2
 * 输入解释：相交节点的值为 2 （注意，如果两个列表相交则不能为 0）。从各自的表头开始算起，链表 A 为 [0,9,1,2,4]，链表 B 为
 * [3,2,4]。在 A 中，相交节点前有 3 个节点；在 B 中，相交节点前有 1 个节点。
 *
 *
 *
 *
 * 示例 3：
 *
 *
 *
 * 输入：intersectVal = 0, listA = [2,6,4], listB = [1,5], skipA = 3, skipB = 2
 * 输出：null
 * 输入解释：从各自的表头开始算起，链表 A 为 [2,6,4]，链表 B 为 [1,5]。由于这两个链表不相交，所以 intersectVal 必须为
 * 0，而 skipA 和 skipB 可以是任意值。
 * 解释：这两个链表不相交，因此返回 null。
 *
 *
 *
 *
 * 注意：
 *
 *
 * 如果两个链表没有交点，返回 null.
 * 在返回结果后，两个链表仍须保持原有的结构。
 * 可假定整个链表结构中没有循环。
 * 程序尽量满足 O(n) 时间复杂度，且仅用 O(1) 内存。
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
 * @param {ListNode} headA
 * @param {ListNode} headB
 * @return {ListNode}
 */
var getIntersectionNode = function(headA, headB) {
  let len1 = getLength(headA);
  let len2 = getLength(headB);
  let lendiff = null;
  let longhead = null;
  let shorthead = null;
  if (len1 >= len2) {
    lendiff = len1 - len2;
    longhead = headA;
    shorthead = headB;
  } else {
    lendiff = len2 - len1;
    longhead = headB;
    shorthead = headA;
  }

  while (lendiff > 0 && longhead) {
    longhead = longhead.next;
    lendiff -= 1;
  }

  while (longhead && shorthead) {
    if (longhead === shorthead) {
      return longhead;
    }
    longhead = longhead.next;
    shorthead = shorthead.next;
  }
  return null;
};

function getLength(head) {
  let count = 0;
  let cur = head;
  while (cur) {
    count += 1;
    cur = cur.next;
  }
  return count;
}
// @lc code=end

function ListNode(val) {
  this.val = val;
  this.next = null;
}

const l1n1 = new ListNode(4);
const l1n2 = new ListNode(1);
const l1n3 = new ListNode(8);
const l1n4 = new ListNode(4);
const l1n5 = new ListNode(5);
l1n1.next = l1n2;
l1n2.next = l1n3;
l1n3.next = l1n4;
l1n4.next = l1n5;

const l2n1 = new ListNode(5);
const l2n2 = new ListNode(0);
const l2n3 = new ListNode(1);
l2n1.next = l2n2;
l2n2.next = l2n3;
l2n3.next = l1n3;

const l1 = l1n1;
const l2 = l2n1;
const result = getIntersectionNode(l1, l2);
console.log(result);
