#include "cpplang.hpp"

class Solution {
 public:
  bool checkPossibility(std::vector<int>& nums) {
    int count = 0;
    int n = nums.size();
    for (int i = 1; i < n; ++i) {
      if (nums[i - 1] > nums[i]) {
        if (i == 1 || nums[i - 2] <= nums[i]) {
          nums[i - 1] = nums[i];
        } else {
          nums[i] = nums[i - 1];
        }
        count += 1;
      }
    }
    return count <= 1;
  }
};

void Test(const char* case_name, std::vector<int>& nums, bool expected_value) {
  Solution solution;
  auto result = solution.checkPossibility(nums);
  if (result == expected_value) {
    printf("%s: pass\n", case_name);
  } else {
    printf("%s: failed\n", case_name);
  }
}

void Test1() {
  std::vector<int> nums{4, 2, 3};
  bool expected_value = true;
  Test("Test1", nums, expected_value);
}

void Test2() {
  std::vector<int> nums{4, 2, 1};
  bool expected_value = false;
  Test("Test2", nums, expected_value);
}

void Test3() {
  std::vector<int> nums{3, 4, 2, 3};
  bool expected_value = false;
  Test("Test3", nums, expected_value);
}

void Test4() {
  std::vector<int> nums{5, 7, 1, 8};
  bool expected_value = true;
  Test("Test4", nums, expected_value);
}

int main() {
  Test1();
  Test2();
  Test3();
  Test4();
  return 0;
}
