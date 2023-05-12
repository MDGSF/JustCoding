#include <iostream>
#include <map>
#include <string>

int main() {
  int n = 0;
  std::cin >> n;

  std::map<std::string, int> m;
  for (int i = 0; i < n; ++i) {
    std::string str;
    std::cin >> str;
    m[str] += 1;
  }

  for (auto item : m) {
    for (int i = 0; i < item.second; ++i) {
      std::cout << item.first << std::endl;
    }
  }
  return 0;
}
