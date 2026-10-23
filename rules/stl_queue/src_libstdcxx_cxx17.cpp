#include <queue>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
typename std::enable_if<std::is_swappable<T2>::value>::type
f9(std::queue<T1, T2> &a0, std::queue<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<
    std::integral_constant<bool, std::is_swappable<T2>::value &&
                                     std::is_swappable<T3>::value>::value>::type
f10(std::priority_queue<T1, T2, T3> &a0, std::priority_queue<T1, T2, T3> &a1) {
  return std::swap<T1, T2, T3>(a0, a1);
}
