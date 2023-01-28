#include "cpplang.hpp"

class Solution {
 public:
  std::vector<int> twoSum(std::vector<int>& nums, int target) {
    std::map<int, int> maps;
    for (int i = 0; i < nums.size(); ++i) {
      int num = nums[i];
      int peer = target - num;
      auto it = maps.find(peer);
      if (it != maps.end()) {
        return std::vector<int>{i, it->second};
      }
      maps[num] = i;
    }
    return std::vector<int>{};
  }
};

void Test(const char* case_name, std::vector<int>& nums, int target,
          std::vector<int>& expected_value) {
  Solution solution;
  auto result = solution.twoSum(nums, target);
  std::sort(result.begin(), result.end());
  std::sort(expected_value.begin(), expected_value.end());
  if (std::equal(result.begin(), result.end(), expected_value.begin(),
                 expected_value.end())) {
    printf("%s: pass\n", case_name);
  } else {
    printf("%s: failed\n", case_name);
  }
}

void Test1() {
  std::vector<int> nums{2, 7, 11, 15};
  int target = 9;
  std::vector<int> expected_value{0, 1};
  Test("Test1", nums, target, expected_value);
}

void Test2() {
  std::vector<int> nums{3, 2, 4};
  int target = 6;
  std::vector<int> expected_value{1, 2};
  Test("Test2", nums, target, expected_value);
}

void Test3() {
  std::vector<int> nums{3, 3};
  int target = 6;
  std::vector<int> expected_value{0, 1};
  Test("Test3", nums, target, expected_value);
}

int main() {
  Test1();
  Test2();
  Test3();
  return 0;
}
