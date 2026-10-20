#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1, typename T2, typename T3>
using t1 = std::ranges::in_in_out_result<T1, T2, T3>;

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::ranges::in_in_out_result<T4, T5, T6>
f1(const std::ranges::in_in_out_result<T1, T2, T3> &a0) {
  return static_cast<std::ranges::in_in_out_result<T4, T5, T6>>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::ranges::in_in_out_result<T4, T5, T6>
f2(std::ranges::in_in_out_result<T1, T2, T3> &&a0) {
  return static_cast<std::ranges::in_in_out_result<T4, T5, T6>>(std::move(a0));
}
