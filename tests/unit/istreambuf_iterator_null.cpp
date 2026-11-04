#include <cassert>
#include <fstream>
#include <iterator>
#include <string>

int main() {
  const std::istreambuf_iterator<char> first(nullptr);
  const std::istreambuf_iterator<char> last(nullptr);
  std::string s(first, last);
  assert(s.empty());
  return 0;
}
