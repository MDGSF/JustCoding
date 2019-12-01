/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} root
 * @return {number}
 */
var diameterOfBinaryTree = function(root) {
  let result = 1;
  var depth = function(node) {
    if (node === null) {
      return 0;
    }
    let left = depth(node.left);
    let right = depth(node.right);
    result = Math.max(result, left + right + 1);
    return Math.max(left, right) + 1;
  };
  depth(root);
  return result - 1;
};
