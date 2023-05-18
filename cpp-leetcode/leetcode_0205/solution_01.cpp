#include <iostream>
#include <map>
#include <string>

class Solution {
 public:
  bool isIsomorphic(std::string s, std::string t) {
    if (s.size() != t.size()) {
      return false;
    }

    std::map<char, int> ms;
    std::map<char, int> mt;
    int count_s = 0;
    int count_t = 0;
    for (int i = 0; i < s.size(); ++i) {
      if (ms.find(s[i]) == ms.end()) {
        count_s += 1;
        ms[s[i]] = count_s;
      }
      if (mt.find(t[i]) == mt.end()) {
        count_t += 1;
        mt[t[i]] = count_t;
      }
      if (ms[s[i]] != mt[t[i]]) {
        return false;
      }
    }

    return true;
  }
};

void Test(const char* case_name, const std::string& s, const std::string& t,
          bool expected_value) {
  Solution solution;
  auto result = solution.isIsomorphic(s, t);
  if (result == expected_value) {
    printf("%s: pass\n", case_name);
  } else {
    printf("%s: failed\n", case_name);
  }
}

void Test1() {
  std::string s = "egg";
  std::string t = "add";
  bool expected_value = true;
  Test("Test1", s, t, expected_value);
}

int main() {
  Test1();
  return 0;
}
