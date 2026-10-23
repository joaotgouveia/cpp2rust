#include <algorithm>
#include <compare>
#include <iterator>
#include <utility>

template <typename T1>
using t6 = typename std::reverse_iterator<T1>::iterator_concept;

template <typename T1>
std::iter_reference_t<T1> f15(const std::reverse_iterator<T1> &a0) {
  return a0.operator*();
}

template <typename T1>
std::reverse_iterator<T1> f16(const std::reverse_iterator<T1> &a0,
                              std::iter_difference_t<T1> a1) {
  return a0.operator+(a1);
}

template <typename T1>
std::reverse_iterator<T1> &f17(std::reverse_iterator<T1> &a0,
                               std::iter_difference_t<T1> a1) {
  return a0.operator+=(a1);
}

template <typename T1>
std::reverse_iterator<T1> f18(const std::reverse_iterator<T1> &a0,
                              std::iter_difference_t<T1> a1) {
  return a0.operator-(a1);
}

template <typename T1>
std::reverse_iterator<T1> &f19(std::reverse_iterator<T1> &a0,
                               std::iter_difference_t<T1> a1) {
  return a0.operator-=(a1);
}

template <typename T1>
std::iter_reference_t<T1> f20(const std::reverse_iterator<T1> &a0,
                              std::iter_difference_t<T1> a1) {
  return a0.operator[](a1);
}

template <typename T1>
std::iter_rvalue_reference_t<T1> f21(const std::reverse_iterator<T1> &a0) {
  return iter_move(a0);
}
