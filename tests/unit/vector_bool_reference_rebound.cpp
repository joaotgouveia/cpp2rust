#include <cassert>
#include <vector>

int main() {
  std::vector<bool> vb(2);
  vb[1] = false;
  assert(!vb[1]);
  vb[0] = true;
  assert(vb[0]);
  vb[1] = vb[0];
  assert(vb[1]);
  bool b = (vb[0] = vb[1]);
  assert(b);
  int i = 0;
  vb[i++] = false;
  assert(!vb[0] && i == 1);
  return 0;
}
