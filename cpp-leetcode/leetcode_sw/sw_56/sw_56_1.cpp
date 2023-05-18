#include <iostream>
#include <vector>
using namespace std;

class Solution {
 public:
  int singleNumber(vector<int>& nums) {
    int aiBitSum[32] = {0};
    for (auto num : nums) {
      int iBitMask = 1;
      for (int i = 31; i >= 0; i--) {
        int iBit = num & iBitMask;
        if (iBit != 0) {
          aiBitSum[i] += 1;
        }
        iBitMask = iBitMask << 1;
      }
    }

    int result = 0;
    for (int i = 0; i < 32; i++) {
      result = result << 1;
      result += aiBitSum[i] % 3;
    }
    return result;
  }
};

int main() {
  std::vector<int> v = {3, 4, 3, 3};
  Solution s;
  int result = s.singleNumber(v);
  printf("result = %d\n", result);
  return 0;
}
