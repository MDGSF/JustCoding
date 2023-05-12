#include <iostream>
#include <string>
#include <set>

int main() {
  std::string str;
  std::cin >> str;
  std::set<char> s(str.begin(), str.end());
  std::cout << s.size() << std::endl;
  return 0;
}
