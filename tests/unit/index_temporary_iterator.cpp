#include <cassert>
#include <vector>

int main() {
  std::vector<int> v{1, 2, 3};
  assert((--v.end())[0] == 3);
  assert(*(--v.end()) == 3);
  return 0;
}
