#include <cassert>
#include <utility>

int main() {
  int a = 1, b = 2;
  int *p = &a, *q = &b;
  std::swap(p, q);
  assert(p == &b && q == &a);
  assert(a == 1 && b == 2);
  int *n = nullptr;
  std::swap(n, p);
  assert(n == &b && p == nullptr);
  return 0;
}
