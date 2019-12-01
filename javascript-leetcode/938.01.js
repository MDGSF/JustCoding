/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} root
 * @param {number} L
 * @param {number} R
 * @return {number}
 */
var rangeSumBST = function(root, L, R) {
  const ctx = {};
  ctx.result = 0;
  dfs(ctx, root, L, R);
  return ctx.result;
};

function dfs(ctx, node, L, R) {
  if (node === null) {
    return;
  }
  if (L <= node.val && node.val <= R) {
    ctx.result += node.val;
  }
  if (L < node.val) {
    dfs(ctx, node.left, L, R);
  }
  if (node.val < R) {
    dfs(ctx, node.right, L, R);
  }
}
