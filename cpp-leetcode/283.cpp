#include <iostream>
#include <vector>
#include <utility>
using namespace std;

class Solution {
public:
  void moveZeroes(vector<int>& nums) {
    int j = 0;
    int n = nums.size();
    for (int i = 0; i < n; i++) {
      if (nums[i] != 0) {
        swap(nums[i], nums[j++]);
      }
    }
  }
};

int main() {
  vector<int> nums = {0, 1, 0, 3, 12};
  Solution o;
  o.moveZeroes(nums);
  for (int i = 0; i < nums.size(); i++) {
    printf("%d ", nums[i]);
  }
  printf("\n");
  return 0;
}
