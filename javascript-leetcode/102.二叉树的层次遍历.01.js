/*
 * @lc app=leetcode.cn id=102 lang=javascript
 *
 * [102] 二叉树的层次遍历
 *
 * https://leetcode-cn.com/problems/binary-tree-level-order-traversal/description/
 *
 * algorithms
 * Medium (53.74%)
 * Likes:    296
 * Dislikes: 0
 * Total Accepted:    51.7K
 * Total Submissions: 87.3K
 * Testcase Example:  '[3,9,20,null,null,15,7]'
 *
 * 给定一个二叉树，返回其按层次遍历的节点值。 （即逐层地，从左到右访问所有节点）。
 *
 * 例如:
 * 给定二叉树: [3,9,20,null,null,15,7],
 *
 * ⁠   3
 * ⁠  / \
 * ⁠ 9  20
 * ⁠   /  \
 * ⁠  15   7
 *
 *
 * 返回其层次遍历结果：
 *
 * [
 * ⁠ [3],
 * ⁠ [9,20],
 * ⁠ [15,7]
 * ]
 *
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
 * @return {number[][]}
 */
var levelOrder = function(root) {
  if (root === null) {
    return [];
  }
  const ctx = {result: []};
  dfs(ctx, root, 0);
  return ctx.result;
};

function dfs(ctx, node, level) {
  if (node === null) {
    return;
  }

  if (ctx.result.length < level + 1) {
    ctx.result.push([]);
  }

  ctx.result[level].push(node.val);

  dfs(ctx, node.left, level + 1);
  dfs(ctx, node.right, level + 1);
}
// @lc code=end
