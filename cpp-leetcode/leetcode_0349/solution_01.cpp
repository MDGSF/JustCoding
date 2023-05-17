#include <algorithm>
#include <iostream>
#include <set>
#include <vector>

class Solution {
 public:
  std::vector<int> intersection(std::vector<int>& nums1,
                                std::vector<int>& nums2) {
    std::vector<int> result;
    std::set<int> s1(nums1.begin(), nums1.end());
    std::set<int> s2(nums2.begin(), nums2.end());
    for (int num : s2) {
      if (s1.find(num) != s1.end()) {
        result.push_back(num);
      }
    }
    return result;
  }
};

void Test(const char* case_name, std::vector<int>& nums1,
          std::vector<int>& nums2, std::vector<int>& expected_value) {
  Solution solution;
  auto result = solution.intersection(nums1, nums2);
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
  std::vector<int> nums1{1, 2, 2, 1};
  std::vector<int> nums2{2, 2};
  std::vector<int> expected_value{2};
  Test("Test1", nums1, nums2, expected_value);
}

void Test2() {
  std::vector<int> nums1{4, 9, 5};
  std::vector<int> nums2{9, 4, 9, 8, 4};
  std::vector<int> expected_value{4, 9};
  Test("Test1", nums1, nums2, expected_value);
}

int main() {
  Test1();
  Test2();
  return 0;
}
