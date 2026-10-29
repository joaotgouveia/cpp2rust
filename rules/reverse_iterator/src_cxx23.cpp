#include <algorithm>
#include <utility>

template <typename T1> using t1 = std::reverse_iterator<T1>;

template <typename T1>
using t2 = typename std::reverse_iterator<T1>::iterator_type;

template <typename T1> using t3 = typename std::reverse_iterator<T1>::pointer;

template <typename T1>
using t4 = typename std::reverse_iterator<T1>::iterator_concept;

template <typename T1>
using t5 = typename std::reverse_iterator<T1>::iterator_category;

template <typename T1>
using t6 = typename std::reverse_iterator<T1>::value_type;

template <typename T1>
using t7 = typename std::reverse_iterator<T1>::difference_type;

template <typename T1> using t8 = typename std::reverse_iterator<T1>::reference;

template <typename T1> std::reverse_iterator<T1> f1() {
  return std::reverse_iterator<T1>();
}

template <typename T1> std::reverse_iterator<T1> f2(T1 a0) {
  return std::reverse_iterator<T1>(std::move(a0));
}

template <typename T1>
std::reverse_iterator<T1> f3(const std::reverse_iterator<T1> &a0) {
  return std::reverse_iterator<T1>(a0);
}

template <typename T1>
std::reverse_iterator<T1> &f4(std::reverse_iterator<T1> &a0,
                              const std::reverse_iterator<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1> T1 f5(const std::reverse_iterator<T1> &a0) {
  return a0.base();
}

template <typename T1>
std::iter_reference_t<T1> f6(const std::reverse_iterator<T1> &a0) {
  return a0.operator*();
}

template <typename T1>
typename std::reverse_iterator<T1>::pointer
f7(const std::reverse_iterator<T1> &a0) {
  return a0.operator->();
}

template <typename T1>
std::reverse_iterator<T1> &f8(std::reverse_iterator<T1> &a0) {
  return a0.operator++();
}

template <typename T1>
std::reverse_iterator<T1> f9(std::reverse_iterator<T1> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
std::reverse_iterator<T1> &f10(std::reverse_iterator<T1> &a0) {
  return a0.operator--();
}

template <typename T1>
std::reverse_iterator<T1> f11(std::reverse_iterator<T1> &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1>
std::reverse_iterator<T1> f12(const std::reverse_iterator<T1> &a0,
                              std::iter_difference_t<T1> a1) {
  return a0.operator+(std::forward<std::iter_difference_t<T1>>(a1));
}

template <typename T1>
std::reverse_iterator<T1> &f13(std::reverse_iterator<T1> &a0,
                               std::iter_difference_t<T1> a1) {
  return a0.operator+=(std::forward<std::iter_difference_t<T1>>(a1));
}

template <typename T1>
std::reverse_iterator<T1> f14(const std::reverse_iterator<T1> &a0,
                              std::iter_difference_t<T1> a1) {
  return a0.operator-(std::forward<std::iter_difference_t<T1>>(a1));
}

template <typename T1>
std::reverse_iterator<T1> &f15(std::reverse_iterator<T1> &a0,
                               std::iter_difference_t<T1> a1) {
  return a0.operator-=(std::forward<std::iter_difference_t<T1>>(a1));
}

template <typename T1>
std::iter_reference_t<T1> f16(const std::reverse_iterator<T1> &a0,
                              std::iter_difference_t<T1> a1) {
  return a0.operator[](std::forward<std::iter_difference_t<T1>>(a1));
}

template <typename T1, typename T2>
std::reverse_iterator<T1> f17(const std::reverse_iterator<T2> &a0)
  requires(!(std::is_same<T2, T1>::value))
{
  return std::reverse_iterator<T1>(a0);
}

template <typename T1, typename T2>
std::reverse_iterator<T1> &f18(std::reverse_iterator<T1> &a0,
                               const std::reverse_iterator<T2> &a1)
  requires(std::assignable_from<T1 &, const T2 &> &&
           (!(std::is_same<T2, T1>::value)))
{
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2>
void f19(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1)
  requires(std::indirectly_swappable<T2, T1>)
{
  return iter_swap(a0, a1);
}

template <typename T1>
std::iter_rvalue_reference_t<T1> f20(const std::reverse_iterator<T1> &a0) {
  return iter_move(a0);
}
