// XFAIL: refcount
#include <cassert>
#include <vector>

int main() {
  std::vector<bool> bits(2, false);
  std::vector<bool>::reference bit = bits[0];
  const std::vector<bool>::reference &assigned = (bit = true);
  assert(assigned);
  assert(bits[0]);
  auto &cleared = bit.operator=(false);
  assert(&cleared == &bit);
  assert(!bits[0]);
  return 0;
}
