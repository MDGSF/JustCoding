#include <algorithm>
#include <iostream>
#include <set>
#include <vector>
#include <map>

class Solution {
  public:
    std::vector<int> intersect(std::vector<int>& nums1, std::vector<int>& nums2) {
      std::map<int, int> counter1;
      for (int num : nums1) {
        counter1[num] += 1;
      }

      std::map<int, int> counter2;
      for (int num : nums2) {
        counter2[num] += 1;
      }

      std::vector<int> result;
      for (auto item : counter1) {
        int num = item.first;
        if (counter2.find(num) != counter2.end()) {
          int min_count = std::min(item.second, counter2[num]);
          for (int i = 0; i < min_count; ++i) {
            result.push_back(num);
          }
        }
      }
      return result;
    }
};

void Test(const char* case_name, std::vector<int>& nums1,
          std::vector<int>& nums2, std::vector<int>& expected_value) {
  Solution solution;
  auto result = solution.intersect(nums1, nums2);
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
  std::vector<int> expected_value{2, 2};
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
