// ADDITIONAL_COMPILE_FLAGS: -std=c++17
#include <assert.h>

int main() {
  int arr[3] = {1, 2, 3};

  auto [a, b, c] = arr;
  a = 10;
  assert(a == 10);
  assert(b == 2);
  assert(c == 3);
  assert(arr[0] == 1);

  auto &[x, y, z] = arr;
  x = 7;
  z += y;
  assert(arr[0] == 7);
  assert(arr[2] == 5);

  double darr[2] = {1.5, 2.5};
  auto [d0, d1] = darr;
  d0 = 4.0;
  assert(d0 + d1 == 6.5);
  assert(darr[0] == 1.5);
  return 0;
}
