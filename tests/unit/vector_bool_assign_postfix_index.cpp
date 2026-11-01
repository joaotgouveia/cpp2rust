// panic: refcount
#include <cassert>
#include <vector>

int main() {
  std::vector<bool> v = {false, false, false};
  int i = 0;
  bool b = v[i++] = true;
  assert(b);
  assert(i == 1);
  assert(v[0]);
  return 0;
}
