#include <cassert>
#include <utility>

int main() {
  std::pair<int, long> a(1, 2L);
  std::pair<short, int> b(7, 8);
  a = b;
  assert(a.first == 7 && a.second == 8);
  b.first = 9;
  assert(a.first == 7);
  return 0;
}
