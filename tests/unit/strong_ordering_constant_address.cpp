#include <cassert>
#include <compare>

int main() {
  const std::strong_ordering &e = std::strong_ordering::equal;
  const std::strong_ordering &g = std::strong_ordering::greater;
  assert(&e == &std::strong_ordering::equal);
  assert(&g == &std::strong_ordering::greater);
  assert(&e != &g);
  assert(&std::strong_ordering::less != &std::strong_ordering::equivalent);
  return 0;
}
