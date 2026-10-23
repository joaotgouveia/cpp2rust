#include <algorithm>
#include <utility>
#include <vector>

template <typename T1>
const typename std::vector<bool, T1>::reference &
f166(const typename std::vector<bool, T1>::reference &a0, bool a1) {
  return a0.operator=(a1);
}
