#include <algorithm>
#include <iostream>
using namespace std;

int main() {
  int n, m;
  cin >> n >> m;
  int arr[m];
  for (int i = 0; i < m; i++) {
    cin >> arr[i];
  }
  sort(arr, arr + m);

  for (int i = 0; i < m; i++) {
    if (n >= arr[i] && n < 100) {
      n++;
    } else {
      break;
    }
  }
  cout << n;
  return 0;
}
