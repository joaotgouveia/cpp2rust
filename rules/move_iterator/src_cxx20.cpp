#include <algorithm>
#include <compare>
#include <iterator>
#include <utility>

template <typename T1>
using t8 = typename std::move_iterator<T1>::iterator_concept;

template <typename T1> const T1 &f13(const std::move_iterator<T1> &a0) {
  return a0.base();
}

template <typename T1> T1 f14(std::move_iterator<T1> &&a0) {
  return std::move(a0).base();
}

template <typename T1>
std::iter_rvalue_reference_t<T1> f15(const std::move_iterator<T1> &a0) {
  return a0.operator*();
}

template <typename T1>
std::move_iterator<T1> f16(const std::move_iterator<T1> &a0,
                           std::iter_difference_t<T1> a1) {
  return a0.operator+(std::forward<std::iter_difference_t<T1>>(a1));
}

template <typename T1>
std::move_iterator<T1> &f17(std::move_iterator<T1> &a0,
                            std::iter_difference_t<T1> a1) {
  return a0.operator+=(std::forward<std::iter_difference_t<T1>>(a1));
}

template <typename T1>
std::move_iterator<T1> f18(const std::move_iterator<T1> &a0,
                           std::iter_difference_t<T1> a1) {
  return a0.operator-(std::forward<std::iter_difference_t<T1>>(a1));
}

template <typename T1>
std::move_iterator<T1> &f19(std::move_iterator<T1> &a0,
                            std::iter_difference_t<T1> a1) {
  return a0.operator-=(std::forward<std::iter_difference_t<T1>>(a1));
}

template <typename T1>
std::iter_rvalue_reference_t<T1> f20(const std::move_iterator<T1> &a0,
                                     std::iter_difference_t<T1> a1) {
  return a0.operator[](std::forward<std::iter_difference_t<T1>>(a1));
}

template <typename T1, typename T2>
void f21(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1)
  requires(std::indirectly_swappable<T2, T1>)
{
  return iter_swap(a0, a1);
}

template <typename T1>
std::iter_rvalue_reference_t<T1> f22(const std::move_iterator<T1> &a0) {
  return iter_move(a0);
}

template <typename T1, typename T2>
std::iter_difference_t<T1> f23(const std::move_iterator<T1> &a0,
                               const std::move_sentinel<T2> &a1)
  requires(std::sized_sentinel_for<T2, T1>)
{
  return operator-(a0, a1);
}

template <typename T1, typename T2>
std::iter_difference_t<T1> f24(const std::move_sentinel<T2> &a0,
                               const std::move_iterator<T1> &a1)
  requires(std::sized_sentinel_for<T2, T1>)
{
  return operator-(a0, a1);
}
