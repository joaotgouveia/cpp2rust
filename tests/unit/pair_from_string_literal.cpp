#include <cassert>
#include <cstring>
#include <string>
#include <utility>

int main() {
  std::pair<const char *, int> p("converted", 99);
  assert(strcmp(p.first, "converted") == 0 && p.second == 99);
  std::pair<std::string, int> s("k", 2);
  assert(s.first == "k" && s.second == 2);
  return 0;
}
