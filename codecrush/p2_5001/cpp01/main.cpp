#include <iostream>
#include <string>

class Solution {
 public:
  void reverse(std::string& s, int start, int end) {
    while (start < end) {
      char t = s[start];
      s[start] = s[end];
      s[end] = t;
      start += 1;
      end -= 1;
    }
  }

  std::string rotate(std::string s, int k) {
    int n = s.size();
    k = k % n;
    reverse(s, 0, n - k - 1);
    reverse(s, n - k, n - 1);
    reverse(s, 0, n - 1);
    return s;
  }
};

void test(const std::string& name, const std::string& s, int k,
          const std::string& expected_result) {
  Solution solu;
  std::string result = solu.rotate(s, k);
  if (result != expected_result) {
    std::cout << name << " failed, " << result << ", " << expected_result
              << std::endl;
  } else {
    std::cout << name << " pass." << std::endl;
  }
}

void test01() { test("test01", "abcdefg", 3, "efgabcd"); }

void test02() { test("test02", "abcdefg", 13, "bcdefga"); }

int main() {
  test01();
  test02();
  return 0;
}
