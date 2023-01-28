#include "tree01.hpp"

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left),
 * right(right) {}
 * };
 */
class Solution {
 public:
  std::vector<int> inorderTraversal(TreeNode* root) {
    std::vector<int> result;
    recursion(result, root);
    return result;
  }

  void recursion(std::vector<int>& result, TreeNode* root) {
    if (root == nullptr) {
      return;
    }
    recursion(result, root->left);
    result.push_back(root->val);
    recursion(result, root->right);
  }
};

int main() { return 0; }
