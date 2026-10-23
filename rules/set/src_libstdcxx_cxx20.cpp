#include <algorithm>
#include <initializer_list>
#include <set>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3, typename T4>
auto f73(const std::set<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template contains<T4>(a1);
}
