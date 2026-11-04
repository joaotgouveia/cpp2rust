#include <cassert>

int main() {
  int x = 0;
  int &r = ++x;
  r = 5;
  assert(x == 5);
  assert(&r == &x);
  --r;
  assert(x == 4);
  (++x) = 7;
  assert(x == 7);
  int y = ++x;
  assert(y == 8 && x == 8);
  unsigned counter = 0;
  unsigned &pre = --counter;
  assert(&pre == &counter && counter == ~0u);
  int a[3] = {1, 2, 3};
  int *p = a;
  int *&q = ++p;
  assert(&q == &p);
  assert(*q == 2);
  assert(*++p == 3);
  int *n = --p;
  assert(*n == 2);
  int z = 7;
  int &w = ++z;
  w = 1;
  assert(z == 1);
  int sum = 0;
  for (int k = 0; k < 3; ++k) {
    sum += ++z;
  }
  assert(sum == 9 && z == 4);
  return 0;
}
