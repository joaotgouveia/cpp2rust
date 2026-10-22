#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1>
std::move_iterator<T1> f14(typename std::move_iterator<T1>::iterator_type a0) {
  return std::move_iterator<T1>(a0);
}

template <typename T1>
typename std::move_iterator<T1>::iterator_type
f15(const std::move_iterator<T1> &a0) {
  return a0.base();
}

template <typename T1>
typename std::move_iterator<T1>::pointer f16(const std::move_iterator<T1> &a0) {
  return a0.operator->();
}

template <typename T1, typename T2>
std::move_iterator<T1> f17(const std::move_iterator<T2> &a0) {
  return std::move_iterator<T1>(a0);
}

template <typename T1, typename T2>
std::move_iterator<T1> &f18(std::move_iterator<T1> &a0,
                            const std::move_iterator<T2> &a1) {
  return a0.template operator= <T2>(a1);
}
