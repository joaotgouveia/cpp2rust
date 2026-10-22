#include "../common/init.h"
#include "../common/traits.h"
#include <iterator>
#include <utility>

template <typename T1, typename T2>
decltype(auto) f16(const std::common_iterator<T1, T2> &a0) {
  return iter_move(a0);
}
