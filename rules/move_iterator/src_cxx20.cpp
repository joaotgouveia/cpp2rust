#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1>
using t8 = typename std::move_iterator<T1>::iterator_concept;

template <typename T1>
const typename std::move_iterator<T1>::iterator_type &
f17(const std::move_iterator<T1> &a0) {
  return a0.base();
}

template <typename T1>
typename std::move_iterator<T1>::iterator_type
f18(std::move_iterator<T1> &&a0) {
  return std::move(a0).base();
}

template <typename T1> void f19(std::move_iterator<T1> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
std::iter_rvalue_reference_t<T1> f20(const std::move_iterator<T1> &a0) {
  return iter_move(a0);
}
