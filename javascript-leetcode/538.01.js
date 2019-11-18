/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} root
 * @return {TreeNode}
 */
var convertBST = function(root) {
  let sum = 0;
  var f = function(root) {
    if (root !== null) {
      f(root.right, sum);
      sum += root.val;
      root.val = sum;
      f(root.left, sum);
    }
    return root;
  };
  return f(root);
};
