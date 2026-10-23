#include <compare>
#include <queue>
#include <ranges>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<std::uses_allocator<T2, T4>::value, int>::type
              T5 = 0>
std::queue<T1, T2> f47(std::from_range_t a0, T3 &&a1, const T4 &a2) {
  return std::queue<T1, T2>(std::move(a0), std::forward<T3>(a1), a2);
}
