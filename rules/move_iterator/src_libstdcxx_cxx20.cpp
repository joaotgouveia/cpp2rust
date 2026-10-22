#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1>
const typename std::move_iterator<T1>::iterator_type &
f19(const std::move_iterator<T1> &a0) {
  return a0.base();
}

template <typename T1>
typename std::move_iterator<T1>::iterator_type
f20(std::move_iterator<T1> &&a0) {
  return std::move(a0).base();
}
