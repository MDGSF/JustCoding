#include <stdio.h>

int isMultiplesOf3Or5(int i) {
  if (i % 3 == 0 || i % 5 == 0) {
    return 1;
  }
  return 0;
}

double SumOfAll3Or5(int n) {
  double sum = 0;
  for (int i = 1; i < n; i++) {
    if (isMultiplesOf3Or5(i)) {
      sum += i;
    }
  }
  return sum;
}

int main() {
  printf("%lf\n", SumOfAll3Or5(10));
  printf("%lf\n", SumOfAll3Or5(1000));
  return 0;
}
