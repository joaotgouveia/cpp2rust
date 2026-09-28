#include <cassert>
#include <string>
#include <vector>

int main() {
  std::vector<std::string> v1 = {"a"};
  assert(v1.back().at(0) == 'a');
  return 0;
}
