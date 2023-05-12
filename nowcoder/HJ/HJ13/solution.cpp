#include <algorithm>
#include <iostream>
#include <string>

void reverse(std::string& str, int start, int end) {
  while (start < end) {
    std::swap(str[start], str[end]);
    start += 1;
    end -= 1;
  }
}

int main() {
  std::string str;
  getline(std::cin, str);
  int start = 0;
  for (int i = 0; i < str.size(); ++i) {
    if (str[i] == ' ') {
      reverse(str, start, i - 1);
      start = i + 1;
    }
  }
  reverse(str, start, str.size() - 1);  // 最后一个单词
  reverse(str, 0, str.size() - 1);      // 整个字符串
  std::cout << str << std::endl;
  return 0;
}
