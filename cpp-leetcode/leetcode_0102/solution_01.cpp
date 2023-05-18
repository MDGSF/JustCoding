#include "cpplang.hpp"
#include "tree01.hpp"

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */
class Solution {
  public:
    std::vector<std::vector<int>> levelOrder(TreeNode* root) {
      if (root == nullptr) {
        return {};
      }

      std::vector<std::vector<int>> result;
      std::deque<TreeNode*> queue;
      queue.push_back(root);
      while (!queue.empty()) {
        int level_size = queue.size();
        std::vector<int> current_level;
        for (int i = 0; i < level_size; ++i) {
          TreeNode* node = queue.front();
          queue.pop_front();
          current_level.push_back(node->val);
          if (node->left != nullptr) {
            queue.push_back(node->left);
          }
          if (node->right != nullptr) {
            queue.push_back(node->right);
          }
        }
        result.push_back(current_level);
      }
      return result;
    }
};

int main() {
  return 0;
}
