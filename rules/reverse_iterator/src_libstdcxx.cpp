#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1>
std::reverse_iterator<T1>
f16(typename std::reverse_iterator<T1>::iterator_type a0) {
  return std::reverse_iterator<T1>(a0);
}

template <typename T1>
std::reverse_iterator<T1> f17(const std::reverse_iterator<T1> &a0) {
  return std::reverse_iterator<T1>(a0);
}

template <typename T1>
std::reverse_iterator<T1> &f18(std::reverse_iterator<T1> &a0,
                               const std::reverse_iterator<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
typename std::reverse_iterator<T1>::iterator_type
f19(const std::reverse_iterator<T1> &a0) {
  return a0.base();
}
