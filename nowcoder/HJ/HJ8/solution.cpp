#include <iostream>
#include <map>

int main() {
  int n = 0;
  std::cin >> n;

  std::map<int, int> m;
  for (int i = 0; i < n; ++i) {
    int index = 0;
    int value = 0;
    std::cin >> index >> value;
    auto it = m.find(index);
    if (it == m.end()) {
      m[index] = value;
    } else {
      it->second += value;
    }
  }

  for (auto it = m.begin(); it != m.end(); ++it) {
    std::cout << it->first << " " << it->second << std::endl;
  }

  return 0;
}
