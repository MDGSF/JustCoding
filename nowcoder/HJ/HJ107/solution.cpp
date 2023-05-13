#include <iostream>
#include <cmath>

double CubeRoot(double x) {
  if (x == 0.0) {
    return 0.0;
  }
  double result = x / 3.0;
  double last_result = result;
  while (true) {
    result = (2.0 * last_result + x / (last_result * last_result)) / 3.0;
    if (std::abs(result - last_result) < 1e-10) {
      break;
    }
    last_result = result;
  }
  return result;
}

int main() {
  double f;
  std::cin >> f;
  printf("%.1lf\n", CubeRoot(f));
  return 0;
}
