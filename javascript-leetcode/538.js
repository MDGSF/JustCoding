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
  let node = root;
  let stack = [];

  while (stack.length > 0 || node !== null) {
    while (node !== null) {
      stack.push(node);
      node = node.right;
    }

    node = stack.pop();
    sum += node.val;
    node.val = sum;

    node = node.left;
  }

  return root;
};
