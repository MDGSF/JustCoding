#include <iostream>
#include <string>
#include <vector>

int main() {
  std::string str;
  getline(std::cin, str);

  std::vector<std::string> words;
  std::string word;
  for (auto c : str) {
    if (std::isalpha(c)) {
      word.push_back(c);
    } else {
      if (!word.empty()) {
        words.push_back(word);
      }
      word.clear();
    }
  }

  if (!word.empty()) {
    words.push_back(word);
  }

  for (int i = words.size() - 1; i >= 0; --i) {
    std::cout << words[i] << " ";
  }
  return 0;
}
