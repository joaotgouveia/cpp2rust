#include "../common/init.h"
#include "../common/traits.h"
#include <chrono>
#include <string_view>
#include <utility>

template <typename T1, typename T2, typename T3>
bool f27(const std::chrono::zoned_time<T1, T3> &a0,
         const std::chrono::zoned_time<T2, T3> &a1) {
  return a0 == a1;
}

using t3 = std::chrono::zoned_seconds;
