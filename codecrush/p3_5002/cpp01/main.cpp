#include <iostream>
#include <string>
#include <vector>

class Solution {
 public:
  int removeDuplicates(std::vector<int>& nums) {
    int n = nums.size();
    int j = 0;
    for (int i = 0; i < n; ++i) {
      if (nums[i] != nums[j]) {
        nums[++j] = nums[i];
      }
    }
    return j + 1;
  }
};

void test(const std::string& name, std::vector<int>& nums,
          const std::vector<int>& expected_nums, int expected_k) {
  Solution solu;
  int k = solu.removeDuplicates(nums);

  bool success = true;
  if (k != expected_k) {
    success = false;
  }
  for (int i = 0; i < k; ++i) {
    if (nums[i] != expected_nums[i]) {
      success = false;
      break;
    }
  }

  if (!success) {
    std::cout << name << " failed." << std::endl;
  } else {
    std::cout << name << " pass." << std::endl;
  }
}

void test01() {
  std::vector<int> nums = {2, 2, 3, 6, 6, 6, 7};
  std::vector<int> expected_nums = {2, 3, 6, 7};
  int expected_k = 4;
  test("test01", nums, expected_nums, expected_k);
}

int main() {
  test01();
  return 0;
}
