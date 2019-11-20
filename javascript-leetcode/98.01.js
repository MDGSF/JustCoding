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
var isValidBST = function(root) {
  return inorder({prev: null}, root);
};

function inorder(ctx, root) {
  if (root === null) {
    return true;
  }
  if (!inorder(ctx, root.left)) {
    return false;
  }
  if (ctx.prev && ctx.prev.val >= root.val) {
    return false;
  }
  ctx.prev = root;
  return inorder(ctx, root.right);
}
