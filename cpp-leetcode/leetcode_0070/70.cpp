#include <iostream>
#include <vector>
#include <utility>
using namespace std;

class Solution {
public:
  int climbStairs(int n) {
    if (n < 3) {
      return n;
    }
    int f1 = 1;
    int f2 = 2;
    for (int i = 2; i < n; i++) {
      int next = f1 + f2;
      f1 = f2;
      f2 = next;
    }
    return f2;
  }
};

int main() {
  Solution o;
  int result = o.climbStairs(3);
  printf("result = %d\n", result);
  return 0;
}
