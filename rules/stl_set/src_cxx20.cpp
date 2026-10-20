#include "../common/init.h"
#include "../common/traits.h"
#include <set>
#include <utility>

template <typename T1, typename T2, typename T3>
auto f8(const std::set<T1, T2, T3> &a0, const std::set<T1, T2, T3> &a1) {
  return a0 <=> a1;
}
