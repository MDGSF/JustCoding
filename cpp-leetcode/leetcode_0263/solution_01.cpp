#include <iostream>

class Solution {
  public:
    bool isUgly(int n) {
      if (n < 1) {
        return false;
      }
      while (n % 2 == 0) {
        n /= 2;
      }
      while (n % 3 == 0) {
        n /= 3;
      }
      while (n % 5 == 0) {
        n /= 5;
      }
      return n == 1;
    }
};

void Test(const char* case_name, int n, bool expected_value) {
  Solution solution;
  auto result = solution.isUgly(n);
  if (result == expected_value) {
    printf("%s: pass\n", case_name);
  } else {
    printf("%s: failed\n", case_name);
  }
}

void Test1() {
  int n = 6;
  bool expected_value = true;
  Test("Test1", n, expected_value);
}

void Test2() {
  int n = 1;
  bool expected_value = true;
  Test("Test2", n, expected_value);
}

void Test3() {
  int n = 14;
  bool expected_value = false;
  Test("Test3", n, expected_value);
}

int main() {
  Test1();
  Test2();
  Test3();
  return 0;
}
