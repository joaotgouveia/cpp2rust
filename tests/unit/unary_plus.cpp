#include <cassert>

struct Counter {
  int x;
};

int main() {
  Counter c = Counter{+1};
  assert(c.x == 1);
  c = Counter{+2};
  assert(c.x == 2);
  int n = +c.x;
  assert(n == 2);
  double d = +1.5;
  assert(d == 1.5);
  assert(+n + +n == 4);
  return 0;
}
