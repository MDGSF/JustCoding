#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

class Solution {
 public:
  int singleNumber(vector<int>& nums) {
    sort(nums.begin(), nums.end());
    for (int i = 0; i < nums.size(); i++) {
      if (nums[i] != nums[i - 1] && nums[i] != nums[i + 1]) {
        return nums[i];
      }
    }
    return nums[0];
  }
};

int main() {
  std::vector<int> v = {3, 4, 3, 3};
  Solution s;
  int result = s.singleNumber(v);
  printf("result = %d\n", result);
  return 0;
}
