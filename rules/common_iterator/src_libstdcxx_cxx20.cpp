#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1, typename T2>
std::common_iterator<T1, T2> f11(const std::common_iterator<T1, T2> &a0) {
  return std::common_iterator<T1, T2>(a0);
}

template <typename T1, typename T2>
std::common_iterator<T1, T2> f12(std::common_iterator<T1, T2> &&a0) {
  return std::common_iterator<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::common_iterator<T1, T2> &f13(std::common_iterator<T1, T2> &a0,
                                  const std::common_iterator<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::common_iterator<T1, T2> &f14(std::common_iterator<T1, T2> &a0,
                                  std::common_iterator<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
std::iter_rvalue_reference_t<T1> f15(const std::common_iterator<T1, T2> &a0) {
  return iter_move(a0);
}
