#include <cassert>
#include <deque>

int main() {
  std::deque<int> d;
  int &x = d.emplace_back(1);
  assert(x == 1);
  d.emplace_back(2);
  int &y = d.emplace_back(3);
  y = 30;
  assert(d.back() == 30);
  assert(d.front() == 1);
  return 0;
}
