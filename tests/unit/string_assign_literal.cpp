#include <cassert>
#include <string>
#include <vector>

int main() {
  std::vector<std::string> v;
  std::string s = "hi";
  v.push_back(s);
  s = "changed";
  assert(v[0] == "hi");
  assert(s == "changed");
  assert(s.size() == 7);
  return 0;
}
