#include <chrono>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <
    typename T1, typename T2, typename T3,
    typename T4 = typename std::enable_if<
        std::integral_constant<bool, std::is_convertible<T3, T2>::value>::value,
        void>::type>
std::chrono::time_point<T1, T2>
f12(const std::chrono::time_point<
    typename std::chrono::time_point<T1, T2>::clock, T3> &a0) {
  return std::chrono::time_point<T1, T2>(a0);
}
