#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1> using t1 = std::ranges::min_max_result<T1>;

template <typename T1, typename T2>
std::ranges::min_max_result<T2> f1(const std::ranges::min_max_result<T1> &a0) {
  return static_cast<std::ranges::min_max_result<T2>>(a0);
}

template <typename T1, typename T2>
std::ranges::min_max_result<T2> f2(std::ranges::min_max_result<T1> &&a0) {
  return static_cast<std::ranges::min_max_result<T2>>(std::move(a0));
}
