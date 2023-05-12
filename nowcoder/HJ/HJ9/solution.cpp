#include <algorithm>
#include <iostream>
#include <set>

int main() {
  std::string n;
  std::cin >> n;

  std::set<char> s;
  reverse(n.begin(), n.end());
  for (int i = 0; i < n.size(); ++i) {
    if (s.find(n[i]) == s.end()) {
      s.insert(n[i]);
      std::cout << n[i];
    }
  }

  return 0;
}
