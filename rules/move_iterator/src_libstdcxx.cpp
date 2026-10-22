#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1>
std::move_iterator<T1> f16(typename std::move_iterator<T1>::iterator_type a0) {
  return std::move_iterator<T1>(a0);
}

template <typename T1>
typename std::move_iterator<T1>::iterator_type
f17(const std::move_iterator<T1> &a0) {
  return a0.base();
}

template <typename T1>
typename std::move_iterator<T1>::pointer f18(const std::move_iterator<T1> &a0) {
  return a0.operator->();
}
