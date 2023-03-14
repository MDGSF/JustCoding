#include "cpplang.hpp"

class Solution {
 public:
  int minNumberOfHours(int initialEnergy, int initialExperience,
                       std::vector<int>& energy, std::vector<int>& experience) {
    int result = 0;

    int energy_sum = 0;
    std::for_each(energy.begin(), energy.end(),
                  [&](int n) { energy_sum += n; });
    if (energy_sum + 1 > initialEnergy) {
      result += (energy_sum + 1 - initialEnergy);
    }

    int experience_sum = initialExperience;
    for (int i = 0; i < experience.size(); ++i) {
      if (experience_sum <= experience[i]) {
        int current = experience[i] + 1 - experience_sum;
        result += current;
        experience_sum = (experience[i] + 1 + experience[i]);
      } else {
        experience_sum += experience[i];
      }
    }

    return result;
  }
};

void Test(const char* case_name, int initialEnergy, int initialExperience,
          std::vector<int>& energy, std::vector<int>& experience,
          int expected_value) {
  Solution solution;
  auto result = solution.minNumberOfHours(initialEnergy, initialExperience,
                                          energy, experience);
  if (result == expected_value) {
    printf("%s: pass\n", case_name);
  } else {
    printf("%s: failed, result:%d, expected_value:%d\n", case_name, result,
           expected_value);
  }
}

void Test1() {
  int initialEnergy = 5;
  int initialExperience = 3;
  std::vector<int> energy = {1, 4, 3, 2};
  std::vector<int> experience = {2, 6, 3, 1};
  int expected_value = 8;
  Test("Test1", initialEnergy, initialExperience, energy, experience,
       expected_value);
}

void Test2() {
  int initialEnergy = 2;
  int initialExperience = 4;
  std::vector<int> energy = {1};
  std::vector<int> experience = {3};
  int expected_value = 0;
  Test("Test2", initialEnergy, initialExperience, energy, experience,
       expected_value);
}

void Test3() {
  int initialEnergy = 1;
  int initialExperience = 1;
  std::vector<int> energy = {1, 1, 1, 1};       // 4
  std::vector<int> experience = {1, 1, 1, 50};  // 1 + 0 + 0 + 46
  int expected_value = 51;
  Test("Test3", initialEnergy, initialExperience, energy, experience,
       expected_value);
}

void Test4() {
  int initialEnergy = 11;
  int initialExperience = 23;
  std::vector<int> energy = {69, 22, 93, 68, 57, 76, 54, 72, 8,  78, 88,
                             15, 58, 61, 25, 70, 58, 91, 79, 22, 91, 74,
                             90, 75, 31, 53, 31, 7,  51, 96, 76, 17, 64,
                             89, 83, 54, 28, 33, 32, 45, 20};
  std::vector<int> experience = {51, 81, 46, 80, 56, 7,  46, 74, 64, 20, 84,
                                 66, 13, 91, 49, 30, 75, 43, 74, 88, 82, 51,
                                 72, 4,  80, 30, 10, 19, 40, 27, 21, 71, 24,
                                 94, 79, 13, 40, 28, 63, 85, 70};
  int expected_value = 2323;
  Test("Test4", initialEnergy, initialExperience, energy, experience,
       expected_value);
}

int main() {
  Test1();
  Test2();
  Test3();
  Test4();
  return 0;
}
