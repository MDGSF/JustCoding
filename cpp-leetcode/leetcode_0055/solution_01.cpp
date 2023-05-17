#include "cpplang.hpp"

class Solution {
 public:
  bool canJump(std::vector<int>& nums) {
    int n = nums.size();
    int max_index = 0;
    for (int i = 0; i < n; ++i) {
      if (max_index >= i) {
        int num = nums[i];
        int cur_max_indx = i + num;
        max_index = std::max(max_index, cur_max_indx);
      }
    }
    return max_index >= n - 1;
  }
};

void Test(const char* case_name, std::vector<int>& nums, bool expected_value) {
  Solution solution;
  bool result = solution.canJump(nums);
  if (result == expected_value) {
    printf("%s: pass\n", case_name);
  } else {
    printf("%s: failed\n", case_name);
  }
}

void Test1() {
  std::vector<int> nums{2, 3, 1, 1, 4};
  bool expected_value = true;
  Test("Test1", nums, expected_value);
}

void Test2() {
  std::vector<int> nums{3, 2, 1, 0, 4};
  bool expected_value = false;
  Test("Test2", nums, expected_value);
}

int main() {
  Test1();
  Test2();
  return 0;
}
