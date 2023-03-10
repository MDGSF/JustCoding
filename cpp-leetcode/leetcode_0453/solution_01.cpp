#include "cpplang.hpp"

class Solution {
 public:
  int minMoves(std::vector<int>& nums) {
    int min = *std::min_element(nums.begin(), nums.end());
    int sum = 0;
    for_each(nums.begin(), nums.end(), [&](int num) { sum += (num - min); });
    return sum;
  }
};

void Test(const char* case_name, std::vector<int>& nums, int expected_value) {
  Solution solution;
  int result = solution.minMoves(nums);
  if (result == expected_value) {
    printf("%s: pass\n", case_name);
  } else {
    printf("%s: failed\n", case_name);
  }
}

void Test1() {
  std::vector<int> nums{1, 2, 3};
  int expected_value = 3;
  Test("Test1", nums, expected_value);
}

void Test2() {
  std::vector<int> nums{1, 1, 1};
  int expected_value = 0;
  Test("Test2", nums, expected_value);
}

void Test3() {
  std::vector<int> nums{0};
  int expected_value = 0;
  Test("Test3", nums, expected_value);
}

void Test4() {
  std::vector<int> nums{1, 1, 1000000000};
  int expected_value = 999999999;
  Test("Test4", nums, expected_value);
}

void Test5() {
  std::vector<int> nums{-100, 0, 100};
  int expected_value = 300;
  Test("Test5", nums, expected_value);
}

int main() {
  Test1();
  Test2();
  Test3();
  Test4();
  Test5();
  return 0;
}
