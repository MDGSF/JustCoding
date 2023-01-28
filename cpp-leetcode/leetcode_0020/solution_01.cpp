#include "cpplang.hpp"

class Solution {
 public:
  bool isValid(std::string s) {
    std::vector<char> chars;
    for (auto it = s.begin(); it != s.end(); ++it) {
      char c = *it;
      if (c == '(' || c == '[' || c == '{') {
        chars.push_back(c);
      } else {
        if (chars.empty()) {
          return false;
        }
        if (c == ')' && chars[chars.size() - 1] != '(') {
          return false;
        }
        if (c == ']' && chars[chars.size() - 1] != '[') {
          return false;
        }
        if (c == '}' && chars[chars.size() - 1] != '{') {
          return false;
        }
        chars.pop_back();
      }
    }
    return chars.empty();
  }
};

void Test(const char* case_name, const std::string& s, bool expected_value) {
  Solution solution;
  auto result = solution.isValid(s);
  if (result == expected_value) {
    printf("%s: pass\n", case_name);
  } else {
    printf("%s: failed\n", case_name);
  }
}

void Test1() {
	std::string s = "()";
  bool expected_value = true;
  Test("Test1", s, expected_value);
}

void Test2() {
	std::string s = "()[]{}";
  bool expected_value = true;
  Test("Test2", s, expected_value);
}

void Test3() {
	std::string s = "(]";
  bool expected_value = false;
  Test("Test3", s, expected_value);
}

void Test4() {
	std::string s = "(";
  bool expected_value = false;
  Test("Test4", s, expected_value);
}

void Test5() {
	std::string s = "[";
  bool expected_value = false;
  Test("Test5", s, expected_value);
}

void Test6() {
	std::string s = "{";
  bool expected_value = false;
  Test("Test6", s, expected_value);
}

void Test7() {
	std::string s = "";
  bool expected_value = true;
  Test("Test7", s, expected_value);
}

int main() {
  Test1();
  Test2();
  Test3();
  Test4();
  Test5();
  Test6();
  Test7();
  return 0;
}
