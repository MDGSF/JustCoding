#include <iostream>
#include <string>
#include <vector>

class Solution {
 public:
  int strStr(std::string haystack, std::string needle) {
    if (needle.empty()) {
      return 0;
    }

    int i = 0;
    int j = 0;
    int n = haystack.size();
    int m = needle.size();
    std::vector<int> next = getNext(needle);

    while (i < n && j < m) {
      if (j == -1 || haystack[i] == needle[j]) {
        i += 1;
        j += 1;
      } else {
        j = next[j];
      }
    }

    if (j == m) {
      return i - j;
    }
    return -1;
  }

  std::vector<int> getNext(const std::string& needle) {
    int m = needle.size();
    std::vector<int> next(m, 0);
    next[0] = -1;
    int k = -1;
    int j = 0;
    while (j < m - 1) {
      if (k == -1 || needle[j] == needle[k]) {
        k += 1;
        j += 1;
        if (needle[j] != needle[k]) {
          next[j] = k;
        } else {
          next[j] = next[k];
        }
      } else {
        k = next[k];
      }
    }
    return next;
  }
};

void Test(const char* case_name, const std::string& haystack,
          const std::string& needle, int expected_value) {
  Solution solution;
  int result = solution.strStr(haystack, needle);
  if (result == expected_value) {
    printf("%s: pass\n", case_name);
  } else {
    printf("%s: failed\n", case_name);
  }
}

void Test1() {
  std::string haystack = "sadbutsad";
  std::string needle = "sad";
  int expected_value = 0;
  Test("Test1", haystack, needle, expected_value);
}

void Test2() {
  std::string haystack = "leetcode";
  std::string needle = "leeto";
  int expected_value = -1;
  Test("Test2", haystack, needle, expected_value);
}

int main() {
  Test1();
  Test2();
  return 0;
}
