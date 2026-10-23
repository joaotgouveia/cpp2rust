#include <algorithm>
#include <utility>

template <typename T1, typename T2>
typename std::enable_if<
    std::integral_constant<bool, std::is_swappable<T1>::value &&
                                     std::is_swappable<T2>::value>::value>::type
f26(std::pair<T1, T2> &a0, std::pair<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}
