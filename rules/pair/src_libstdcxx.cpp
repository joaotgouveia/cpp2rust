#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<
    std::integral_constant<
        bool, std::is_assignable<T1 &, const T3 &>::value &&
                  std::is_assignable<T2 &, const T4 &>::value>::value,
    std::pair<T1, T2> &>::type
f11(std::pair<T1, T2> &a0, const std::pair<T3, T4> &a1) {
  return a0.template operator= <T3, T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<
    std::integral_constant<bool,
                           std::is_assignable<T1 &, T3 &&>::value &&
                               std::is_assignable<T2 &, T4 &&>::value>::value,
    std::pair<T1, T2> &>::type
f12(std::pair<T1, T2> &a0, std::pair<T3, T4> &&a1) {
  return a0.template operator= <T3, T4>(std::move(a1));
}
