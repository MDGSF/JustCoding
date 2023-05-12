#include <iostream>

int main() {
  std::string str;
  while (std::cin >> str) {
    int len = str.length();
    int cnt = 0;
    if (len % 8 == 0) {
      cnt = len / 8;
    } else {
      cnt = len / 8 + 1;
      str += std::string(8 - len % 8, '0');
    }

    for (int i = 0; i < cnt; ++i) {
      std::cout << str.substr(i * 8, 8) << std::endl;
    }
  }
  return 0;
}
