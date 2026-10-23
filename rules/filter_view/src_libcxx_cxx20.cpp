#include <ranges>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3 = T1>
T1 f23(const std::ranges::filter_view<T1, T2> &a0) {
  return a0.base();
}
