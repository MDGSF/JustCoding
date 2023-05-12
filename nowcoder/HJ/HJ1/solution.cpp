#include <iostream>
#include <string>

int main() {
  std::string str;
  getline(std::cin, str);
  int pos = str.rfind(' ');
  int len = str.length() - pos - 1;
  std::cout << len << std::endl;
  return 0;
}
