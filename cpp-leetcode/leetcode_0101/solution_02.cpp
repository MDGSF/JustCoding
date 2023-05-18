#include <iostream>
#include <deque>

struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode *left, TreeNode *right)
      : val(x), left(left), right(right) {}
};

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
  bool isSymmetric(TreeNode *root) {
    std::deque<TreeNode*> queue;
    queue.push_back(root);
    queue.push_back(root);
    while (!queue.empty()) {
      TreeNode* n1 = queue.front();
      queue.pop_front();
      TreeNode* n2 = queue.front();
      queue.pop_front();
      if (n1 == nullptr && n2 == nullptr) {
        continue;
      } else if (n1 == nullptr || n2 == nullptr) {
        return false;
      } else if (n1->val != n2->val) {
        return false;
      }
      queue.push_back(n1->left);
      queue.push_back(n2->right);
      queue.push_back(n1->right);
      queue.push_back(n2->left);
    }
    return true;
  }
};

int main() { return 0; }
