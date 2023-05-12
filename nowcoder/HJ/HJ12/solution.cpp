#include <algorithm>
#include <iostream>
#include <string>

int main() {
  std::string str;
  getline(std::cin, str);
  reverse(str.begin(), str.end());
  std::cout << str << std::endl;
  return 0;
}

