#include <algorithm>
#include <iostream>
#include <queue>
#include <vector>

class Solution {
 public:
  long long kSum(std::vector<int>& nums, int k) {
    long long sum = 0;
    for (int& num : nums) {
      if (num >= 0) {
        sum += num;
      } else {
        num = -num;
      }
    }
    std::sort(nums.begin(), nums.end());
    std::priority_queue<std::pair<long long, int>> pq;
    pq.emplace(sum, 0);
    while (--k) {
      auto t = pq.top();
      long long sum = t.first;
      int i = t.second;
      pq.pop();
      if (i < nums.size()) {
        pq.emplace(sum - nums[i], i + 1);
        if (i > 0) {
          pq.emplace(sum - nums[i] + nums[i - 1], i + 1);
        }
      }
    }
    return pq.top().first;
  }
};

void Test(const char* case_name, std::vector<int>& nums, int k,
          long long expected_value) {
  Solution solution;
  auto result = solution.kSum(nums, k);
  if (result == expected_value) {
    printf("%s: pass\n", case_name);
  } else {
    printf("%s: failed\n", case_name);
  }
}

void Test1() {
  std::vector<int> nums{2, 4, -2};
  int k = 5;
  long long expected_value = 2;
  Test("Test1", nums, k, expected_value);
}

int main() {
  Test1();
  return 0;
}
