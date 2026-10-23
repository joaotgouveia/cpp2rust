#include <array>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, std::size_t T2,
          typename std::enable_if<T2 == 0 || std::is_swappable_v<T1>, int>::type
              T3 = 0>
void f42(std::array<T1, T2> &a0, std::array<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}
