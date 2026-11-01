#include <cassert>
#include <vector>

int main() {
  std::vector<bool> v1(2, false);
  std::vector<bool> v2(2, true);

  assert(v1[0] == v1[1]);
  assert(v2[0] == v2[1]);

  assert(v1[0] != v2[0]);
  assert(v1[1] != v2[1]);

  return 0;
}
