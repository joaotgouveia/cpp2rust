#include <cassert>

constexpr int square(int x) { return x * x; }

int main() {
  constexpr bool ce = __builtin_is_constant_evaluated();
  assert(ce);
  bool rt = __builtin_is_constant_evaluated();
  assert(!rt);
  constexpr int n = square(-3) - 10;
  assert(n == -1);
  constexpr double d = 2.5 * 2;
  assert(d == 5.0);
  constexpr unsigned long big = 1UL << 40;
  assert(big == 1099511627776UL);
  return 0;
}
