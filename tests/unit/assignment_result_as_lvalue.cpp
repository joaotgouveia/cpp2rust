#include <cassert>
#include <utility>

int main() {
  int a = 1;
  int b = 42;
  int &r = (a = std::move(b));
  assert(a == 42);
  assert(&r == &a);
  int arr[5] = {1, 2, 3, 4, 5};
  int *p = arr;
  int *&pr = (p += 3);
  assert(*pr == 4);
  assert(&pr == &p);
  (a = b) = 5;
  assert(a == 5);
  assert(b == 42);
  r = 7;
  assert(a == 7);
  return 0;
}
