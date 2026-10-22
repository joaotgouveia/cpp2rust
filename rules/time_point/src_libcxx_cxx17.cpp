#include "../common/init.h"
#include "../common/traits.h"
#include <chrono>
#include <utility>

template <typename T1, typename T2,
          std::enable_if_t<std::numeric_limits<T1>::is_signed, int> T3 = 0>
std::chrono::duration<T1, T2> f24(std::chrono::duration<T1, T2> a0) {
  return std::chrono::abs<T1, T2>(a0);
}
