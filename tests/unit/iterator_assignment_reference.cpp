#include <cassert>
#include <map>
#include <utility>
#include <vector>

int main() {
  std::map<int, int> m;
  m[1] = 1;
  m[2] = 2;
  auto it = m.begin();
  std::map<int, int>::iterator saved;
  std::map<int, int>::iterator &res = (saved = std::move(it));
  assert(&res == &saved);
  assert(saved->first == 1);
  std::map<int, int>::iterator &adv = ++saved;
  assert(&adv == &saved);
  assert(saved->first == 2);
  std::map<int, int>::iterator copied;
  std::map<int, int>::iterator &cres = (copied = saved);
  assert(&cres == &copied);
  assert(copied->first == 2);

  std::vector<int> v{5, 6};
  std::vector<int>::iterator vit;
  std::vector<int>::iterator &vres = (vit = v.begin() + 1);
  assert(&vres == &vit);
  assert(*vit == 6);
  std::vector<int>::iterator moved;
  std::vector<int>::iterator &mres = (moved = std::move(vit));
  assert(&mres == &moved);
  assert(*moved == 6);
  return 0;
}
