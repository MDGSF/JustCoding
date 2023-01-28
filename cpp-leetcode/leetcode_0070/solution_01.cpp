#include "cpplang.hpp"

class Solution {
 public:
  int climbStairs(int n) {
    if (n < 3) {
      return n;
    }
    int f1 = 1;
    int f2 = 2;
    for (int i = 2; i < n; ++i) {
      int t = f1 + f2;
      f1 = f2;
      f2 = t;
    }
    return f2;
  }
};

void Test(const char* case_name, int n, int expected_value) {
  Solution solution;
  auto result = solution.climbStairs(n);
  if (result == expected_value) {
    printf("%s: pass\n", case_name);
  } else {
    printf("%s: failed\n", case_name);
  }
}

void Test1() {
  int n = 1;
  int expected_value = 1;
  Test("Test1", n, expected_value);
}

void Test2() {
  int n = 2;
  int expected_value = 2;
  Test("Test2", n, expected_value);
}

void Test3() {
  int n = 3;
  int expected_value = 3;
  Test("Test3", n, expected_value);
}

int main() {
  Test1();
  Test2();
  Test3();
  return 0;
}
