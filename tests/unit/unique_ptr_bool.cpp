#include <cassert>
#include <memory>

int main() {
  std::unique_ptr<int> u(new int(3));
  assert(u);
  std::unique_ptr<int> e;
  assert(!e);
  if (u) {
    *u = 4;
  }
  assert(*u == 4);
  auto a = std::make_unique<int[]>(2);
  assert(a);
  std::unique_ptr<int[]> ea;
  assert(!ea);
  return 0;
}
