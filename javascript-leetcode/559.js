/**
 * // Definition for a Node.
 * function Node(val,children) {
 *    this.val = val;
 *    this.children = children;
 * };
 */
/**
 * @param {Node} root
 * @return {number}
 */
var maxDepth = function(root) {
  if (root === null) {
    return 0;
  }
  let maxdepth = 0;
  for (let i = 0; i < root.children.length; i += 1) {
    let curDepth = maxDepth(root.children[i]);
    maxdepth = Math.max(maxdepth, curDepth);
  }
  return maxdepth + 1;
};
