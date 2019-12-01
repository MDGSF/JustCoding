/*
 * @lc app=leetcode.cn id=101 lang=javascript
 *
 * [101] 对称二叉树
 *
 * https://leetcode-cn.com/problems/symmetric-tree/description/
 *
 * algorithms
 * Easy (45.21%)
 * Likes:    475
 * Dislikes: 0
 * Total Accepted:    63.9K
 * Total Submissions: 130.8K
 * Testcase Example:  '[1,2,2,3,4,4,3]'
 *
 * 给定一个二叉树，检查它是否是镜像对称的。
 *
 * 例如，二叉树 [1,2,2,3,4,4,3] 是对称的。
 *
 * ⁠   1
 * ⁠  / \
 * ⁠ 2   2
 * ⁠/ \ / \
 * 3  4 4  3
 *
 *
 * 但是下面这个 [1,2,2,null,3,null,3] 则不是镜像对称的:
 *
 * ⁠   1
 * ⁠  / \
 * ⁠ 2   2
 * ⁠  \   \
 * ⁠  3    3
 *
 *
 * 说明:
 *
 * 如果你可以运用递归和迭代两种方法解决这个问题，会很加分。
 *
 */

// @lc code=start
/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} root
 * @return {boolean}
 */
var isSymmetric = function(root) {
  let q = [];
  q.push(root);
  q.push(root);
  while (q.length > 0) {
    let t1 = q.shift();
    let t2 = q.shift();
    if (t1 === null && t2 === null) {
      continue;
    }
    if (t1 === null || t2 === null) {
      return false;
    }
    if (t1.val !== t2.val) {
      return false;
    }
    q.push(t1.left);
    q.push(t2.right);
    q.push(t1.right);
    q.push(t2.left);
  }
  return true;
};

// @lc code=end
