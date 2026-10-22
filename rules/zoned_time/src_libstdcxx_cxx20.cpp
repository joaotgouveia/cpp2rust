#include "../common/init.h"
#include "../common/traits.h"
#include <chrono>
#include <string_view>
#include <utility>

template <typename T1, typename T2>
bool f26(const std::chrono::zoned_time<T1, T2> &a0,
         const std::chrono::zoned_time<T1, T2> &a1) {
  return a0 == a1;
}
