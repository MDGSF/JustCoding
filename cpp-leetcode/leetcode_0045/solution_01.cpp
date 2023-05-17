#include "cpplang.hpp"

class Solution {
 public:
  int jump(std::vector<int>& nums) {
    int end = 0;
    int steps = 0;
    int max_index = 0;
    for (int i = 0; i < nums.size() - 1; ++i) {
      int num = nums[i];
      int cur_max_index = i + num;
      max_index = std::max(max_index, cur_max_index);
      if (i == end) {
        end = max_index;
        steps += 1;
      }
    }
    return steps;
  }
};

void Test(const char* case_name, std::vector<int>& nums, int expected_value) {
  Solution solution;
  int result = solution.jump(nums);
  if (result == expected_value) {
    printf("%s: pass\n", case_name);
  } else {
    printf("%s: failed, result: %d, expected_value: %d\n", case_name, result,
           expected_value);
  }
}

void Test1() {
  std::vector<int> nums{2, 3, 1, 1, 4};
  int expected_value = 2;
  Test("Test1", nums, expected_value);
}

void Test2() {
  std::vector<int> nums{2, 3, 0, 1, 4};
  int expected_value = 2;
  Test("Test2", nums, expected_value);
}

void Test3() {
  std::vector<int> nums{2, 3, 1};
  int expected_value = 1;
  Test("Test3", nums, expected_value);
}

void Test4() {
  std::vector<int> nums{7,0,9,6,9,6,1,7,9,0,1,2,9,0,3};
  int expected_value = 2;
  Test("Test4", nums, expected_value);
}

int main() {
  Test1();
  Test2();
  Test3();
  Test4();
  return 0;
}
