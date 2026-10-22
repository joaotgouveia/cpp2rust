#include "../common/init.h"
#include "../common/traits.h"
#include <queue>
#include <utility>
#include <vector>

template <typename T1, typename T2>
bool f6(const std::vector<T1, T2> &a0, const std::vector<T1, T2> &a1) {
  return a0 == a1;
}
