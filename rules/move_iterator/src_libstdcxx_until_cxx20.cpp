#include <algorithm>
#include <utility>

template <typename T1>
typename std::move_iterator<T1>::iterator_type
f18(const std::move_iterator<T1> &a0) {
  return a0.base();
}
