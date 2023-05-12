#include <iostream>
#include <string>

int main() {
  std::string str;
  std::cin >> str;

  // 统计每个字符出现次数
  int count[26] = {0};
  for (auto c : str) {
    count[c - 'a'] += 1;
  }

  // 找到出现最少的次数
  int min_value = 100;
  for (int i = 0; i < 26; ++i) {
    if (count[i] > 0 && count[i] < min_value) {
      min_value = count[i];
    }
  }

  for (auto c : str) {
    if (count[c - 'a'] > min_value) {
      std::cout << c;
    }
  }

  return 0;
}
