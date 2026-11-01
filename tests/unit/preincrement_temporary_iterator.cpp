#include <cassert>
#include <map>

int main() {
  std::map<int, int> m;
  m[1] = 10;
  m[2] = 20;
  m.erase(++m.begin());
  assert(m.size() == 1);
  assert(m.begin()->first == 1);
  return 0;
}
