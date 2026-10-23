#include <chrono>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t10 = std::chrono::days;

using t11 = std::chrono::months;

template <typename T1, typename T2, typename T3, typename T4>
auto f38(const std::chrono::duration<T1, T2> &a0,
         const std::chrono::duration<T3, T4> &a1) {
  return a0 <=> a1;
}

using t12 = std::chrono::weeks;

using t13 = std::chrono::years;
