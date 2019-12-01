/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} root
 * @param {number} sum
 * @return {number}
 */
var pathSum = function(root, sum) {
  if (root === null) {
    return 0;
  }
  return (
    pathSumFrom(root, sum) + pathSum(root.left, sum) + pathSum(root.right, sum)
  );
};

function pathSumFrom(node, sum) {
  if (node === null) {
    return 0;
  }
  return (
    (node.val === sum ? 1 : 0) +
    pathSumFrom(node.left, sum - node.val) +
    pathSumFrom(node.right, sum - node.val)
  );
}
