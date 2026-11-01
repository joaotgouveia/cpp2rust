#include <cassert>
#include <vector>

int main() {
  std::vector<int> v = {10, 20, 30, 40};
  auto i = v.end();
  auto &r = (i -= 3);
  assert(&r == &i);
  assert(*i == 20);
  auto &s = ++i;
  assert(&s == &i);
  assert(*i == 30);
  auto &t = --i;
  assert(&t == &i);
  assert(*i == 20);
  auto &u = (i += 2);
  assert(&u == &i);
  assert(*i == 40);
  auto j = v.begin();
  int x = *++j;
  assert(x == 20);
  auto k = ++j;
  assert(*k == 30);
  assert(*j == 30);
  return 0;
}
