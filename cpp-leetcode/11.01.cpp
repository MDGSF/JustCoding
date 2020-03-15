#include <iostream>
#include <vector>
#include <utility>
using namespace std;

class Solution {
public:
  int maxArea(vector<int>& height) {
    int result = 0;
    for (int i = 0; i < height.size() - 1; i++) {
      for (int j = 0; j < height.size(); j++) {
        int curHeight = min(height[i], height[j]);
        int curWidth = j - i;
        int curArea = curHeight * curWidth;
        result = max(result, curArea);
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
