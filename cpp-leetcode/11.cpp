#include <iostream>
#include <vector>
#include <utility>
using namespace std;

class Solution {
public:
  int maxArea(vector<int>& height) {
    int result = 0;
    int left = 0;
    int right = height.size() - 1;
    while (left < right) {
      int curHeight = min(height[left], height[right]);
      int curWidth = right - left;
      int curArea = curHeight * curWidth;
      result = max(result, curArea);
      if (height[left] < height[right]) {
        left += 1;
      } else {
        right -= 1;
      }
    }
    return result;
  }
};

int main() {
  vector<int> height = {1,8,6,2,5,4,8,3,7};
  Solution o;
  int result = o.maxArea(height);
  printf("result = %d\n", result);
  return 0;
}
