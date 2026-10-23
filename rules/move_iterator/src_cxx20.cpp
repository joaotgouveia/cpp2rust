#include <algorithm>
#include <compare>
#include <iterator>
#include <utility>

template <typename T1>
using t8 = typename std::move_iterator<T1>::iterator_concept;

template <typename T1> void f11(std::move_iterator<T1> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
std::iter_rvalue_reference_t<T1> f12(const std::move_iterator<T1> &a0) {
  return iter_move(a0);
}
