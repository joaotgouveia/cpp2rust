#include <iterator>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3 = T1>
auto f16(const std::common_iterator<T1, T2> &a0) {
  return a0.operator->();
}

template <typename T1, typename T2>
decltype(auto) f17(const std::common_iterator<T1, T2> &a0) {
  return iter_move(a0);
}
